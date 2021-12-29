use anyhow::anyhow;
use futures::Sink;
use futures::SinkExt;
use futures::StreamExt;
use houseflow_accessory_hal::Accessory;
use houseflow_config::accessory::Credentials;
use houseflow_types::hive;
use houseflow_types::hive::AccessoryFrame;
use houseflow_types::hive::ExecuteResultFrame;
use houseflow_types::hive::HubFrame;
use houseflow_types::hive::StateFrame;
use reqwest::Url;
use std::borrow::Cow;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite;
use tungstenite::Message as WebsocketMessage;

const PING_TIMEOUT: Duration = Duration::from_secs(10);
const PING_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub enum Event {
    #[allow(dead_code)]
    Ping,
    Pong,
    Close(Option<tungstenite::protocol::CloseFrame<'static>>),
    AccessoryFrame(hive::AccessoryFrame),
}

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;

pub struct Session {
    heartbeat: Mutex<Instant>,
    hub_url: Url,
    credentials: Credentials,
}

impl Session {
    pub fn new(hub_url: Url, credentials: Credentials) -> Self {
        Self {
            heartbeat: Mutex::new(Instant::now()),
            hub_url,
            credentials
        }
    }

    pub async fn run(self, accessory: &impl Accessory) -> Result<(), anyhow::Error> {
        let url = self.hub_url.join("/ws").unwrap();

        let http_request = http::Request::builder()
            .uri(url.to_string())
            .header(
                http::header::AUTHORIZATION,
                format!(
                    "Basic {}",
                    base64::encode(format!("{}:{}", self.credentials.id, self.credentials.id))
                ),
            )
            .body(())
            .unwrap();

        let (stream, _) = tokio_tungstenite::connect_async(http_request).await?;
        let (event_sender, event_receiver) = mpsc::channel::<Event>(8);
        let (stream_sender, stream_receiver) = stream.split();

        tokio::select! {
            v = self.stream_read(stream_receiver, event_sender.clone(), accessory) => { v }
            v = self.stream_write(stream_sender, event_receiver) => { v }
            v = self.heartbeat(event_sender.clone()) => { 
                event_sender.send(Event::Close(Some(tungstenite::protocol::CloseFrame{
                    code: tungstenite::protocol::frame::coding::CloseCode::Error,
                    reason: Cow::from("heartbeat timeout"),
                }))).await?;
                v
             }
        }
    }

    async fn stream_read<S, A>(
        &self,
        mut stream: S,
        events: EventSender,
        accessory: &A,
    ) -> anyhow::Result<()>
    where
        S: futures::Stream<Item = Result<WebsocketMessage, tungstenite::Error>> + Unpin,
        A: Accessory,
    {
        while let Some(message) = stream.next().await {
            let message = message?;
            match message {
                WebsocketMessage::Text(text) => {
                    tracing::debug!("Raw frame: `{}`", text);
                    let frame: HubFrame = serde_json::from_str(&text)?;
                    tracing::debug!("Parsed frame: {:?}", frame);
                    match frame {
                        HubFrame::Execute(frame) => {
                            let status = accessory.execute(frame.command).await?;
                            let state = accessory.state().await?;
                            let frame = ExecuteResultFrame {
                                id: frame.id,
                                status,
                                state,
                            };
                            let frame = AccessoryFrame::ExecuteResult(frame);
                            let event = Event::AccessoryFrame(frame);
                            events.send(event).await.expect("failed sending event");
                        }
                        HubFrame::Query(_) => {
                            let state = accessory.state().await?;
                            let frame = StateFrame { state };
                            let frame = AccessoryFrame::State(frame);
                            let response_event = Event::AccessoryFrame(frame);
                            events
                                .send(response_event)
                                .await
                                .expect("failed sending event");
                        }
                        _ => {
                            panic!("Unexpected frame received")
                        }
                    }
                }
                WebsocketMessage::Binary(bytes) => {
                    tracing::debug!("received binary: {:?}", bytes);
                }
                WebsocketMessage::Ping(payload) => {
                    tracing::debug!("received ping, payload: {:?}", payload);
                    events
                        .send(Event::Pong)
                        .await
                        .expect("message receiver half is down");
                }
                WebsocketMessage::Pong(payload) => {
                    tracing::debug!("Received pong, payload: {:?}", payload);
                    *self.heartbeat.lock().await = Instant::now();
                }
                WebsocketMessage::Close(frame) => {
                    tracing::info!("Received close frame: {:?}", frame);
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    async fn stream_write<S>(
        &self,
        mut stream: S,
        mut events: EventReceiver,
    ) -> Result<(), anyhow::Error>
    where
        S: Sink<WebsocketMessage, Error = tungstenite::Error> + Unpin,
    {
        while let Some(event) = events.recv().await {
            match event {
                Event::Ping => {
                    tracing::debug!("sending ping message");
                    stream.send(WebsocketMessage::Ping(Vec::new())).await?;
                }
                Event::Pong => {
                    tracing::debug!("sending pong message");
                    stream.send(WebsocketMessage::Pong(Vec::new())).await?;
                }
                Event::AccessoryFrame(frame) => {
                    let json = serde_json::to_string(&frame).unwrap();
                    tracing::debug!("sending text message: {}", json);
                    stream.send(WebsocketMessage::Text(json)).await?;
                }
                Event::Close(frame) => {
                    tracing::debug!("sending close frame: {:?}", frame);
                    stream.send(WebsocketMessage::Close(frame)).await?;
                    stream.close().await?;
                },
            }
        }
        Ok(())
    }

    async fn heartbeat(&self, events: EventSender) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(PING_INTERVAL);
            interval.tick().await;
        loop {
            interval.tick().await;
            if Instant::now().duration_since(*self.heartbeat.lock().await) > PING_TIMEOUT {
                return Err(anyhow!("server heartbeat failed"));
            } else {
                events.send(Event::Ping).await?;
            }
        }
    }
}
