use axum::extract::ws::Message;
use houseflow_types::{
    errors::InternalError,
    lighthouse::proto::{execute, execute_response, query, state, Frame},
};
use tokio::sync::{broadcast, mpsc};

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
enum Request {
    Execute(execute::Frame),
    Query(query::Frame),
}

#[derive(Debug, Clone)]
enum Response {
    Execute(execute_response::Frame),
    Query(state::Frame),
}

#[derive(Debug, Clone)]
pub struct Session {
    requests: mpsc::UnboundedSender<Request>,
    responses: broadcast::Sender<Response>,
}

// Non-clonable session internals
#[derive(Debug)]
pub struct SessionInternals {
    requests: (
        mpsc::UnboundedSender<Request>,
        mpsc::UnboundedReceiver<Request>,
    ),
    responses: (broadcast::Sender<Response>, broadcast::Receiver<Response>),
}

impl Default for SessionInternals {
    fn default() -> Self {
        Self {
            requests: mpsc::unbounded_channel(),
            responses: broadcast::channel(512),
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
            requests: internals.requests.0.clone(),
            responses: internals.responses.0.clone(),
        }
    }

    pub async fn run(
        &self,
        stream: impl futures::Stream<Item = Result<Message, axum::Error>>
            + futures::Sink<Message, Error = axum::Error>
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
        S: futures::Stream<Item = Result<Message, axum::Error>> + Unpin,
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
                            self.responses.send(Response::Query(state))?;
                        }
                        Frame::ExecuteResponse(execute_response) => {
                            self.responses.send(Response::Execute(execute_response))?;
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
        S: futures::Sink<Message, Error = axum::Error> + Unpin,
    {
        async fn send_json<S, T>(stream: &mut S, val: &T) -> Result<(), SessionError>
        where
            S: futures::Sink<Message, Error = axum::Error> + Unpin,
            T: serde::Serialize,
        {
            let json = serde_json::to_string(val)?;
            stream
                .send(Message::Text(json))
                .await
                .map_err(SessionError::WebsocketError)
        }

        while let Some(request) = internals.requests.1.recv().await {
            match request {
                Request::Execute(execute) => {
                    send_json(&mut stream, &Frame::Execute(execute)).await?
                }
                Request::Query(query) => send_json(&mut stream, &Frame::Query(query)).await?,
            }
        }
        Ok(())
    }

    pub async fn execute(
        &self,
        frame: execute::Frame,
    ) -> Result<execute_response::Frame, InternalError> {
        let mut response_subscriber = self.responses.subscribe();
        let frame_id = frame.id;
        self.requests
            .send(Request::Execute(frame))
            .map_err(|err| InternalError::Other(err.to_string()))?;

        loop {
            let response = response_subscriber
                .recv()
                .await
                .map_err(|err| InternalError::Other(err.to_string()))?;
            if let Response::Execute(execute_response) = response {
                if execute_response.id == frame_id {
                    break Ok::<_, InternalError>(execute_response);
                }
            }
        }
    }

    pub async fn query(&self, frame: query::Frame) -> Result<state::Frame, InternalError> {
        let mut response_subscriber = self.responses.subscribe();
        self.requests
            .send(Request::Query(frame))
            .map_err(|err| InternalError::Other(err.to_string()))?;

        loop {
            if let Response::Query(query_respose) = response_subscriber
                .recv()
                .await
                .map_err(|err| InternalError::Other(err.to_string()))?
            {
                return Ok(query_respose);
            }
        }
    }
}
