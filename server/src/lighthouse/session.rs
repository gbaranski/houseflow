use axum::extract::ws::Message as WebSocketMessage;
use houseflow_types::errors::InternalError;
use houseflow_types::lighthouse;
use houseflow_types::lighthouse::HubFrame;
use houseflow_types::lighthouse::ServerFrame;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

const PING_INTERVAL: Duration = Duration::from_secs(5);
const PING_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("websocket error: {0}")]
    WebsocketError(axum::Error),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("send message over channel failed")]
    SendOverChannelError(String),
    #[error("heartbeat failed")]
    HeartbeatFailed,
}

impl<T> From<tokio::sync::broadcast::error::SendError<T>> for SessionError {
    fn from(val: tokio::sync::broadcast::error::SendError<T>) -> Self {
        Self::SendOverChannelError(val.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for SessionError {
    fn from(val: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::SendOverChannelError(val.to_string())
    }
}

#[derive(Debug, Clone)]
enum ServerMessage {
    Frame(ServerFrame),
    Ping(Vec<u8>),
}

#[derive(Debug, Clone)]
enum HubMessage {
    Frame(HubFrame),
}

#[derive(Debug, Clone)]
pub struct Session {
    server_messages: mpsc::UnboundedSender<ServerMessage>,
    hub_messages: broadcast::Sender<HubMessage>,
    last_heartbeat: Arc<Mutex<Instant>>,
}

// Non-clonable session internals
#[derive(Debug)]
pub struct SessionInternals {
    server_messages: (
        mpsc::UnboundedSender<ServerMessage>,
        mpsc::UnboundedReceiver<ServerMessage>,
    ),
    hub_messages: (
        broadcast::Sender<HubMessage>,
        broadcast::Receiver<HubMessage>,
    ),
}

impl Default for SessionInternals {
    fn default() -> Self {
        Self {
            server_messages: mpsc::unbounded_channel(),
            hub_messages: broadcast::channel(512),
        }
    }
}

impl SessionInternals {
    pub fn new() -> Self {
        Self::default()
    }
}

use futures::SinkExt;
use futures::StreamExt;

impl Session {
    pub fn new(internals: &SessionInternals) -> Self {
        Self {
            server_messages: internals.server_messages.0.clone(),
            hub_messages: internals.hub_messages.0.clone(),
            last_heartbeat: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub async fn run(
        &self,
        stream: impl futures::Stream<Item = Result<WebSocketMessage, axum::Error>>
            + futures::Sink<WebSocketMessage, Error = axum::Error>
            + Unpin,
        internals: SessionInternals,
    ) -> Result<(), tower::BoxError> {
        let (tx, rx) = stream.split();
        tokio::select! {
            value = self.stream_read(rx) => value?,
            value = self.stream_write(tx, internals) => value?,
            value = self.heartbeat() => value?,
        };

        Ok(())
    }

    async fn stream_read<S>(&self, mut stream: S) -> Result<(), SessionError>
    where
        S: futures::Stream<Item = Result<WebSocketMessage, axum::Error>> + Unpin,
    {
        while let Some(message) = stream
            .next()
            .await
            .transpose()
            .map_err(SessionError::WebsocketError)?
        {
            match message {
                WebSocketMessage::Text(text) => {
                    tracing::debug!("Text message received: `{}`", text);
                    let frame = serde_json::from_str::<HubFrame>(&text)?;
                    self.hub_messages.send(HubMessage::Frame(frame))?;
                }
                WebSocketMessage::Binary(_) => todo!(),
                WebSocketMessage::Ping(_) => {
                    tracing::debug!("Ping received");
                }
                WebSocketMessage::Pong(_bytes) => {
                    tracing::debug!("Pong received");
                    *self.last_heartbeat.lock().unwrap() = Instant::now();
                }
                WebSocketMessage::Close(frame) => {
                    tracing::info!("Close info: {:?}", frame);
                }
            }
        }

        Ok(())
    }

    async fn stream_write<S>(
        &self,
        mut stream: S,
        mut internals: SessionInternals,
    ) -> Result<(), SessionError>
    where
        S: futures::Sink<WebSocketMessage, Error = axum::Error> + Unpin,
    {
        while let Some(request) = internals.server_messages.1.recv().await {
            let message = match request {
                ServerMessage::Frame(frame) => {
                    let text = serde_json::to_string(&frame)?;
                    WebSocketMessage::Text(text)
                }
                ServerMessage::Ping(bytes) => WebSocketMessage::Ping(bytes),
            };

            tracing::debug!("Message `{:?}` sent", message);
            stream
                .send(message)
                .await
                .map_err(SessionError::WebsocketError)?;
        }
        Ok(())
    }

    async fn heartbeat(&self) -> Result<(), SessionError> {
        let mut interval = tokio::time::interval(PING_INTERVAL);
        loop {
            interval.tick().await;
            if Instant::now().duration_since(*self.last_heartbeat.lock().unwrap()) > PING_TIMEOUT {
                return Err(SessionError::HeartbeatFailed);
            }
            self.server_messages.send(ServerMessage::Ping(Vec::new()))?;
        }
    }

    pub async fn accessory_execute(
        &self,
        frame: lighthouse::AccessoryExecuteFrame,
    ) -> Result<lighthouse::AccessoryExecuteResultFrame, InternalError> {
        let mut subscriber = self.hub_messages.subscribe();
        let frame_id = frame.id;
        self.server_messages
            .send(ServerMessage::Frame(ServerFrame::AccessoryExecute(frame)))
            .map_err(|err| InternalError::Other(err.to_string()))?;

        loop {
            let response = subscriber
                .recv()
                .await
                .map_err(|err| InternalError::Other(err.to_string()))?;
            if let HubMessage::Frame(HubFrame::AccessoryExecuteResult(result)) = response
            {
                if result.id == frame_id {
                    break Ok::<_, InternalError>(result);
                }
            }
        }
    }

    pub async fn accessory_query(&self, frame: lighthouse::AccessoryQueryFrame) -> Result<lighthouse::AccessoryUpdateFrame, InternalError> {
        let mut subscriber = self.hub_messages.subscribe();
        self.server_messages
            .send(ServerMessage::Frame(ServerFrame::AccessoryQuery(frame)))
            .map_err(|err| InternalError::Other(err.to_string()))?;

        loop {
            if let HubMessage::Frame(HubFrame::AccessoryUpdate(frame)) = subscriber
                .recv()
                .await
                .map_err(|err| InternalError::Other(err.to_string()))?
            {
                return Ok(frame);
            }
        }
    }

    pub async fn hub_query(&self, frame: lighthouse::HubQueryFrame) -> Result<lighthouse::HubUpdateFrame, InternalError> {
        let mut subscriber = self.hub_messages.subscribe();
        self.server_messages
            .send(ServerMessage::Frame(ServerFrame::HubQuery(frame)))
            .map_err(|err| InternalError::Other(err.to_string()))?;

        loop {
            if let HubMessage::Frame(HubFrame::HubUpdate(frame)) = subscriber
                .recv()
                .await
                .map_err(|err| InternalError::Other(err.to_string()))?
            {
                return Ok(frame);
            }
        }
    }
}
