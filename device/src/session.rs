use crate::Device;
use anyhow::anyhow;
use futures_util::Sink;
use futures_util::SinkExt;
use futures_util::StreamExt;
use houseflow_config::device::Server;
use houseflow_types::lighthouse::execute_response;
use houseflow_types::lighthouse::state;
use houseflow_types::lighthouse::Frame;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tungstenite::Message as WebsocketMessage;
use url::Url;

const PING_TIMEOUT: Duration = Duration::from_secs(10);
const PING_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub enum Event {
    #[allow(dead_code)]
    Ping,

    Pong,
    LighthouseFrame(Frame),
}

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;

pub struct Session {
    heartbeat: Mutex<Instant>,
    server_config: Server,
}

impl Session {
    pub fn new(server_config: Server) -> Self {
        Self {
            server_config,
            heartbeat: Mutex::new(Instant::now()),
        }
    }

    pub async fn run(self, device: impl Device) -> Result<(), anyhow::Error> {
        use houseflow_config::defaults;

        let url = format!(
            "ws{}://{}:{}/lighthouse/ws",
            if self.server_config.use_tls { "s" } else { "" },
            self.server_config.hostname,
            if self.server_config.use_tls {
                defaults::server_port_tls()
            } else {
                defaults::server_port()
            },
        );
        tracing::info!("`{}` will be used the as Server URL", url);
        let url = Url::parse(&url).unwrap();

        tracing::debug!("will use {} as websocket endpoint", url);

        let credentials = device.credentials();
        let http_request = http::Request::builder()
            .uri(url.to_string())
            .header(
                http::header::AUTHORIZATION,
                format!(
                    "Basic {}",
                    base64::encode(format!("{}:{}", credentials.id, credentials.password))
                ),
            )
            .body(())
            .unwrap();

        let (stream, _) = tokio_tungstenite::connect_async(http_request).await?;
        let (event_sender, event_receiver) = mpsc::channel::<Event>(8);
        let (stream_sender, stream_receiver) = stream.split();

        tokio::select! {
            v = self.stream_read(stream_receiver, event_sender.clone(), device) => { v }
            v = self.stream_write(stream_sender, event_receiver) => { v }
            _ = self.heartbeat(event_sender.clone()) => { Err(anyhow::anyhow!("server did not respond to heartbeat")) }
        }
    }

    async fn stream_read<S, D>(
        &self,
        mut stream: S,
        events: EventSender,
        device: D,
    ) -> anyhow::Result<()>
    where
        S: futures_util::Stream<Item = Result<WebsocketMessage, tungstenite::Error>> + Unpin,
        D: Device,
    {
        while let Some(message) = stream.next().await {
            let message = message?;
            match message {
                WebsocketMessage::Text(text) => {
                    tracing::debug!("Raw frame: `{}`", text);
                    let frame: Frame = serde_json::from_str(&text)?;
                    tracing::debug!("Parsed frame: {:?}", frame);
                    match frame {
                        Frame::Execute(frame) => {
                            let status = device.on_command(frame.command).await?;
                            let state = serde_json::to_value(device.state()?)?
                                .as_object()
                                .unwrap()
                                .to_owned();
                            let response_frame = execute_response::Frame {
                                id: frame.id,
                                status,
                                state,
                            };
                            let response_frame = Frame::ExecuteResponse(response_frame);
                            let response_event = Event::LighthouseFrame(response_frame);
                            events
                                .send(response_event)
                                .await
                                .expect("failed sending event");
                        }
                        Frame::Query(_) => {
                            let state = serde_json::to_value(device.state()?)?
                                .as_object()
                                .unwrap()
                                .to_owned();
                            let response_frame = state::Frame { state };
                            let response_frame = Frame::State(response_frame);
                            let response_event = Event::LighthouseFrame(response_frame);
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
                    tracing::debug!("Received ping, payload: {:?}", payload);
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
                    tracing::debug!("Sending Ping");
                    stream.send(WebsocketMessage::Ping(Vec::new())).await?;
                }
                Event::Pong => {
                    tracing::debug!("Sending Pong");
                    stream.send(WebsocketMessage::Pong(Vec::new())).await?;
                }
                Event::LighthouseFrame(frame) => {
                    let json = serde_json::to_string(&frame).unwrap();
                    tracing::debug!("sending text message: {}", json);
                    stream.send(WebsocketMessage::Text(json)).await?;
                }
            }
        }
        Ok(())
    }

    async fn heartbeat(&self, events: EventSender) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(PING_INTERVAL);
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
