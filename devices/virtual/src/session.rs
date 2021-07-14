use crate::devices;
use futures_util::{Sink, SinkExt, StreamExt};
use houseflow_config::device::Config;
use houseflow_types::lighthouse::proto::{execute_response, state, Frame};
use tokio::sync::mpsc;
use tungstenite::Message as WebsocketMessage;
use url::Url;

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
    config: Config,
}

impl Session {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run<D: devices::Device<EP>, EP: devices::ExecuteParams>(
        self,
        device: D,
    ) -> Result<(), anyhow::Error> {
        let url = format!(
            "ws{}://{}:{}/lighthouse/ws",
            if self.config.use_tls { "s" } else { "" },
            self.config.server_hostname,
            houseflow_config::defaults::server_port(),
        );
        tracing::info!("`{}` will be used the as Server URL", url);
        let url = Url::parse(&url).unwrap();

        tracing::debug!("will use {} as websocket endpoint", url);
        let http_request = http::Request::builder()
            .uri(url.to_string())
            .header(
                http::header::AUTHORIZATION,
                format!(
                    "Basic {}:{}",
                    self.config.device_id, self.config.device_password
                ),
            )
            .body(())
            .unwrap();

        let (stream, _) = tokio_tungstenite::connect_async(http_request).await?;
        let (event_sender, event_receiver) = mpsc::channel::<Event>(8);
        let (stream_sender, stream_receiver) = stream.split();

        tokio::select! {
            v = self.stream_read(stream_receiver, event_sender, device) => { v }
            v = self.stream_write(stream_sender, event_receiver) => { v }
        }
    }

    async fn stream_read<S, D: devices::Device<EP>, EP: devices::ExecuteParams>(
        &self,
        mut stream: S,
        events: EventSender,
        mut device: D,
    ) -> anyhow::Result<()>
    where
        S: futures_util::Stream<Item = Result<WebsocketMessage, tungstenite::Error>> + Unpin,
    {
        while let Some(message) = stream.next().await {
            let message = message?;
            match message {
                WebsocketMessage::Text(text) => {
                    tracing::debug!("received frame: `{}`", text);
                    let frame: Frame = serde_json::from_str(&text)?;
                    tracing::debug!("Received frame: {:?}", frame);
                    match frame {
                        Frame::Execute(frame) => {
                            let params: EP =
                                serde_json::from_value(serde_json::Value::Object(frame.params))?;
                            let (status, error) = device.on_execute(frame.command, params).await?;
                            let response_frame = execute_response::Frame {
                                id: frame.id,
                                status,
                                error,
                                state: device.state(),
                            };
                            let response_frame = Frame::ExecuteResponse(response_frame);
                            let response_event = Event::LighthouseFrame(response_frame);
                            events
                                .send(response_event)
                                .await
                                .expect("failed sending event");
                        }
                        Frame::Query(_) => {
                            let response_frame = state::Frame {
                                state: device.state(),
                            };
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
                    events
                        .send(Event::Pong)
                        .await
                        .expect("message receiver half is down");
                    tracing::info!("Received ping, payload: {:?}", payload);
                }
                WebsocketMessage::Pong(payload) => {
                    tracing::info!("Received ping, payload: {:?}", payload);
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
                    tracing::info!("Sending Ping");
                    stream.send(WebsocketMessage::Ping(Vec::new())).await?;
                }
                Event::Pong => {
                    tracing::info!("Sending Pong");
                    stream.send(WebsocketMessage::Pong(Vec::new())).await?;
                }
                Event::LighthouseFrame(frame) => {
                    let json = serde_json::to_string(&frame).unwrap();
                    stream.send(WebsocketMessage::Text(json)).await?;
                }
            }
        }
        Ok(())
    }
}
