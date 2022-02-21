use anyhow::anyhow;
use futures::Sink;
use futures::SinkExt;
use futures::StreamExt;
use houseflow_accessory_hal::Accessory;
use houseflow_accessory_hal::AccessoryEvent;
use houseflow_accessory_hal::AccessoryEventReceiver;
use houseflow_config::accessory::Credentials;
use houseflow_types::hive;
use houseflow_types::hive::AccessoryFrame;
use houseflow_types::hive::CharacteristicReadResult;
use houseflow_types::hive::CharateristicWriteResult;
use houseflow_types::hive::HubFrame;
use std::borrow::Cow;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite;
use tungstenite::Message as WebsocketMessage;
use url::Url;

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

pub type EventSender = mpsc::UnboundedSender<Event>;
pub type EventReceiver = mpsc::UnboundedReceiver<Event>;

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
            credentials,
        }
    }

    pub async fn run(
        self,
        accessory: &impl Accessory,
        accessory_events: AccessoryEventReceiver,
    ) -> Result<(), anyhow::Error> {
        let url = self.hub_url.join("/provider/hive/websocket").unwrap();

        let http_request = http::Request::builder()
            .uri(url.to_string())
            .header(
                http::header::AUTHORIZATION,
                format!(
                    "Basic {}",
                    base64::encode(format!(
                        "{}:{}",
                        self.credentials.id, self.credentials.password
                    ))
                ),
            )
            .body(())
            .unwrap();

        let (stream, _) = tokio_tungstenite::connect_async(http_request).await?;
        let (event_sender, event_receiver) = mpsc::unbounded_channel::<Event>();
        let (stream_sender, stream_receiver) = stream.split();

        tokio::select! {
            v = self.stream_read(stream_receiver, event_sender.clone(), accessory) => { v }
            v = self.stream_write(stream_sender, event_receiver) => { v }
            v = self.accessory_events_read(accessory_events, event_sender.clone()) => { v }
            v = self.heartbeat(event_sender.clone()) => {
                event_sender.send(Event::Close(Some(tungstenite::protocol::CloseFrame{
                    code: tungstenite::protocol::frame::coding::CloseCode::Error,
                    reason: Cow::from("heartbeat timeout"),
                })))?;
                v
             }
        }
    }

    async fn accessory_events_read(
        &self,
        mut accessory_events: AccessoryEventReceiver,
        events: EventSender,
    ) -> Result<(), anyhow::Error> {
        while let Some(event) = accessory_events.recv().await {
            match event {
                AccessoryEvent::CharacteristicUpdate {
                    service_name,
                    characteristic,
                } => {
                    events.send(Event::AccessoryFrame(AccessoryFrame::UpdateCharacteristic(
                        hive::UpdateCharacteristic {
                            service_name,
                            characteristic,
                        },
                    )))?;
                }
            };
        }
        Ok(())
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
                        HubFrame::ReadCharacteristic(frame) => {
                            let result = accessory
                                .read_characteristic(frame.service_name, frame.characteristic_name)
                                .await;
                            let frame = CharacteristicReadResult {
                                id: frame.id,
                                result: result.into(),
                            };
                            let frame = AccessoryFrame::CharacteristicReadResult(frame);
                            let response_event = Event::AccessoryFrame(frame);
                            events.send(response_event).unwrap()
                        }
                        HubFrame::WriteCharacteristic(frame) => {
                            let result = accessory
                                .write_characteristic(frame.service_name, frame.characteristic)
                                .await;
                            let frame = CharateristicWriteResult {
                                id: frame.id,
                                result: result.into(),
                            };
                            let frame = AccessoryFrame::CharacteristicWriteResult(frame);
                            let event = Event::AccessoryFrame(frame);
                            events.send(event).unwrap()
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
                }
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
                events.send(Event::Ping)?;
            }
        }
    }
}
