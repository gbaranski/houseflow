use futures_util::{Sink, SinkExt, Stream, StreamExt};
use houseflow_types::{DeviceID, DevicePassword};
use lighthouse_proto::frame;
use tokio::sync::mpsc;
use tungstenite::Message as WebsocketMessage;
use url::Url;

#[derive(Debug, Clone)]
pub enum Event {
    Ping,
    Pong,
    // Execute(frame::execute::Frame),
    // ExecuteResponse(frame::execute_response::Frame),
}

pub type EventSender = mpsc::Sender<Event>;
pub type EventReceiver = mpsc::Receiver<Event>;
pub type EventChannel = (EventSender, EventReceiver);

pub struct Options {
    pub url: Url,
    pub id: DeviceID,
    pub password: DevicePassword,
}

pub struct Session {
    opts: Options,
}

impl Session {
    pub fn new(opts: Options) -> Self {
        Self { opts }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        let http_request = http::Request::builder()
            .uri(self.opts.url.to_string())
            .header(
                http::header::AUTHORIZATION,
                format!("Basic {}:{}", self.opts.id, self.opts.password),
            )
            .body(())
            .unwrap();

        let (stream, _) = tokio_tungstenite::connect_async(http_request).await?;
        let (event_sender, event_receiver) = mpsc::channel::<Event>(8);
        let (stream_sender, stream_receiver) = stream.split();

        tokio::select! {
            v = self.stream_read(stream_receiver, event_sender) => { v }
            v = self.stream_write(stream_sender, event_receiver) => { v }
        }
    }

    async fn stream_read<S>(&self, mut stream: S, events: EventSender) -> Result<(), anyhow::Error>
    where
        S: futures_util::Stream<Item = Result<WebsocketMessage, tungstenite::Error>> + Unpin,
    {
        while let Some(message) = stream.next().await {
            let message = message?;
            match message {
                WebsocketMessage::Text(text) => {
                    log::info!("Received text data: {:?}", text);
                }
                WebsocketMessage::Binary(bytes) => {
                    log::info!("Received binary data: {:?}", bytes);
                }
                WebsocketMessage::Ping(payload) => {
                    events
                        .send(Event::Pong)
                        .await
                        .expect("message receiver half is down");
                    log::info!("Received ping, payload: {:?}", payload);
                }
                WebsocketMessage::Pong(payload) => {
                    log::info!("Received ping, payload: {:?}", payload);
                }
                WebsocketMessage::Close(frame) => {
                    log::info!("Received close frame: {:?}", frame);
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
                    log::info!("Sending Ping");
                    stream.send(WebsocketMessage::Ping(Vec::new())).await?;
                }
                Event::Pong => {
                    log::info!("Sending Pong");
                    stream.send(WebsocketMessage::Pong(Vec::new())).await?;
                }
            }
        }
        unimplemented!();
    }
}
