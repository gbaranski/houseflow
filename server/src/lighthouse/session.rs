use axum::extract::ws::Message as WebSocketMessage;
use houseflow_types::{
    errors::InternalError,
    lighthouse::proto::{execute, execute_response, query, state, Frame},
};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::sync::{broadcast, mpsc};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const PING_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("websocket error: {0}")]
    WebsocketError(axum::Error),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("frame `{frame_name}` was not expected in this context")]
    UnexpectedFrame { frame_name: &'static str },

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
    Execute(execute::Frame),
    Query(query::Frame),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
}

#[derive(Debug, Clone)]
enum DeviceMessage {
    ExecuteResponse(execute_response::Frame),
    State(state::Frame),
    // Ping(Vec<u8>),
    // Pong(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Session {
    server_messages: mpsc::UnboundedSender<ServerMessage>,
    device_messages: broadcast::Sender<DeviceMessage>,
    last_heartbeat: Arc<Mutex<Instant>>,
}

// Non-clonable session internals
#[derive(Debug)]
pub struct SessionInternals {
    server_messages: (
        mpsc::UnboundedSender<ServerMessage>,
        mpsc::UnboundedReceiver<ServerMessage>,
    ),
    device_messages: (
        broadcast::Sender<DeviceMessage>,
        broadcast::Receiver<DeviceMessage>,
    ),
}

impl Default for SessionInternals {
    fn default() -> Self {
        Self {
            server_messages: mpsc::unbounded_channel(),
            device_messages: broadcast::channel(512),
        }
    }
}

impl SessionInternals {
    pub fn new() -> Self {
        Self::default()
    }
}

use futures::{SinkExt, StreamExt};

impl Session {
    pub fn new(internals: &SessionInternals) -> Self {
        Self {
            server_messages: internals.server_messages.0.clone(),
            device_messages: internals.device_messages.0.clone(),
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
            _ = self.stream_read(rx) => {},
            _ = self.stream_write(tx, internals) => {}
            _ = self.heartbeat() => {}
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
                    let frame = serde_json::from_str::<Frame>(&text)?;
                    match frame {
                        Frame::State(state) => {
                            self.device_messages.send(DeviceMessage::State(state))?;
                        }
                        Frame::ExecuteResponse(execute_response) => {
                            self.device_messages
                                .send(DeviceMessage::ExecuteResponse(execute_response))?;
                        }
                        Frame::Query(_) | Frame::Execute(_) => {
                            return Err(SessionError::UnexpectedFrame {
                                frame_name: frame.name(),
                            })
                        }
                        _ => unimplemented!(),
                    };
                }
                WebSocketMessage::Binary(_) => todo!(),
                WebSocketMessage::Ping(bytes) => {
                    // self.server_messages.send(ServerMessage::Pong(bytes))?;
                }
                WebSocketMessage::Pong(_bytes) => {
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
                ServerMessage::Execute(frame) => {
                    WebSocketMessage::Text(serde_json::to_string(&Frame::Execute(frame))?)
                }
                ServerMessage::Query(frame) => {
                    WebSocketMessage::Text(serde_json::to_string(&Frame::Query(frame))?)
                }
                ServerMessage::Ping(bytes) => WebSocketMessage::Ping(bytes),
                ServerMessage::Pong(bytes) => WebSocketMessage::Pong(bytes),
            };
            stream
                .send(message)
                .await
                .map_err(SessionError::WebsocketError)?;
        }
        Ok(())
    }

    async fn heartbeat(&self) -> Result<(), SessionError> {
        let mut interval = tokio::time::interval(HEARTBEAT_INTERVAL);
        loop {
            interval.tick().await;
            if Instant::now().duration_since(*self.last_heartbeat.lock().unwrap()) > PING_TIMEOUT {
                return Err(SessionError::HeartbeatFailed);
            }
            self.server_messages.send(ServerMessage::Ping(Vec::new()))?;
        }
    }

    pub async fn execute(
        &self,
        frame: execute::Frame,
    ) -> Result<execute_response::Frame, InternalError> {
        let mut subscriber = self.device_messages.subscribe();
        let frame_id = frame.id;
        self.server_messages
            .send(ServerMessage::Execute(frame))
            .map_err(|err| InternalError::Other(err.to_string()))?;

        loop {
            let response = subscriber
                .recv()
                .await
                .map_err(|err| InternalError::Other(err.to_string()))?;
            if let DeviceMessage::ExecuteResponse(execute_response) = response {
                if execute_response.id == frame_id {
                    break Ok::<_, InternalError>(execute_response);
                }
            }
        }
    }

    pub async fn query(&self, frame: query::Frame) -> Result<state::Frame, InternalError> {
        let mut subscriber = self.device_messages.subscribe();
        self.server_messages
            .send(ServerMessage::Query(frame))
            .map_err(|err| InternalError::Other(err.to_string()))?;

        loop {
            if let DeviceMessage::State(state) = subscriber
                .recv()
                .await
                .map_err(|err| InternalError::Other(err.to_string()))?
            {
                return Ok(state);
            }
        }
    }
}
