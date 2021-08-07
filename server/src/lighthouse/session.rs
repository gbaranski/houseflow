use axum::ws::Message;
use houseflow_types::{
    errors::InternalError,
    lighthouse::proto::{execute, execute_response, query, state, Frame},
};
use tokio::sync::{broadcast, mpsc};

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("websocket error: {0}")]
    WebsocketError(Box<dyn std::error::Error + Send + Sync>),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("frame `{frame_name}` was not expected in this context")]
    UnexpectedFrame { frame_name: &'static str },

    #[error("send message over channel failed")]
    SendOverChannelError(String),
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
pub struct Session {
    execute: mpsc::Sender<execute::Frame>,
    execute_response: broadcast::Sender<execute_response::Frame>,
    query: mpsc::Sender<query::Frame>,
    state: broadcast::Sender<state::Frame>,
}

// Non-clonable session internals
#[derive(Debug)]
pub struct SessionInternals {
    execute: (mpsc::Sender<execute::Frame>, mpsc::Receiver<execute::Frame>),
    execute_respose: (
        broadcast::Sender<execute_response::Frame>,
        broadcast::Receiver<execute_response::Frame>,
    ),
    query: (mpsc::Sender<query::Frame>, mpsc::Receiver<query::Frame>),
    state: (
        broadcast::Sender<state::Frame>,
        broadcast::Receiver<state::Frame>,
    ),
}

impl Default for SessionInternals {
    fn default() -> Self {
        Self {
            execute: mpsc::channel(4),
            execute_respose: broadcast::channel(4),
            query: mpsc::channel(4),
            state: broadcast::channel(4),
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
            execute: internals.execute.0.clone(),
            execute_response: internals.execute_respose.0.clone(),
            query: internals.query.0.clone(),
            state: internals.state.0.clone(),
        }
    }

    pub async fn run(
        &self,
        stream: impl futures::Stream<Item = Result<Message, tower::BoxError>>
            + futures::Sink<Message, Error = tower::BoxError>
            + Unpin,
        internals: SessionInternals,
    ) -> Result<(), tower::BoxError> {
        let (tx, rx) = stream.split();
        tokio::select! {
            _ = self.stream_read(rx) => {},
            _ = self.stream_write(tx, internals) => {}
        };

        Ok(())
    }

    async fn stream_read<S>(&self, mut stream: S) -> Result<(), SessionError>
    where
        S: futures::Stream<Item = Result<Message, tower::BoxError>> + Unpin,
    {
        while let Some(message) = stream
            .next()
            .await
            .transpose()
            .map_err(SessionError::WebsocketError)?
        {
            match message {
                Message::Text(text) => {
                    let frame = serde_json::from_str::<Frame>(&text)?;
                    match frame {
                        Frame::State(state) => {
                            self.state.send(state)?;
                        }
                        Frame::ExecuteResponse(execute_response) => {
                            self.execute_response.send(execute_response)?;
                        }
                        Frame::Query(_) | Frame::Execute(_) => {
                            return Err(SessionError::UnexpectedFrame {
                                frame_name: frame.name(),
                            })
                        }
                        _ => unimplemented!(),
                    };
                }
                Message::Binary(_) => todo!(),
                Message::Ping(_) => todo!(),
                Message::Pong(_) => todo!(),
                Message::Close(_) => todo!(),
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
        S: futures::Sink<Message, Error = tower::BoxError> + Unpin,
    {
        async fn send_json<S, T>(stream: &mut S, val: &T) -> Result<(), SessionError>
        where
            S: futures::Sink<Message, Error = tower::BoxError> + Unpin,
            T: serde::Serialize,
        {
            let json = serde_json::to_string(val)?;
            stream
                .send(Message::Text(json))
                .await
                .map_err(SessionError::WebsocketError)
        }

        loop {
            tokio::select! {
                Some(execute) = internals.execute.1.recv() => {
                    send_json(&mut stream, &execute).await?;
                }
                Some(query) = internals.query.1.recv() => {
                    send_json(&mut stream, &query).await?;
                }
            };
        }
    }

    pub async fn execute(
        &self,
        frame: execute::Frame,
    ) -> Result<execute_response::Frame, InternalError> {
        let mut execute_response_subscriber = self.execute_response.subscribe();
        let frame_id = frame.id;
        self.execute
            .send(frame)
            .await
            .map_err(|err| InternalError::Other(err.to_string()))?;

        loop {
            let execute_response = execute_response_subscriber
                .recv()
                .await
                .map_err(|err| InternalError::Other(err.to_string()))?;
            if execute_response.id == frame_id {
                break Ok::<_, InternalError>(execute_response);
            }
        }
    }

    pub async fn query(&self, frame: query::Frame) -> Result<state::Frame, InternalError> {
        let mut state_subscriber = self.state.subscribe();
        self.query
            .send(frame)
            .await
            .map_err(|err| InternalError::Other(err.to_string()))?;

        state_subscriber
            .recv()
            .await
            .map_err(|err| InternalError::Other(err.to_string()))
    }
}
