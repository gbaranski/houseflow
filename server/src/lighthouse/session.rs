use actix::{Actor, ActorContext, Handler, StreamHandler};
use actix_web_actors::ws;
use houseflow_types::lighthouse::{
    proto::{execute, execute_response, query, state, Frame, FrameID},
    DeviceCommunicationError,
};
use houseflow_types::DeviceID;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::{broadcast, oneshot};

use super::aliases::*;

const EXECUTE_TIMEOUT: Duration = Duration::from_secs(5);
const QUERY_TIMEOUT: Duration = Duration::from_secs(5);
const STATE_CHANNEL_SIZE: usize = 4;

#[derive(Debug, Error)]
pub enum SessionError {
    // #[error("websocket error: {0}")]
// WebsocketError(#[from] warp::Error),
}

pub struct Session {
    device_id: DeviceID,
    address: SocketAddr,
    pub execute_channels: HashMap<FrameID, oneshot::Sender<execute_response::Frame>>,
    pub state_channel: broadcast::Sender<state::Frame>,
}

impl Session {
    pub fn new(device_id: DeviceID, address: SocketAddr) -> Self {
        let (state_channel, _) = broadcast::channel(STATE_CHANNEL_SIZE);

        Self {
            device_id,
            address,
            state_channel,
            execute_channels: Default::default(),
        }
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        tracing::info!(
            "New device connected from {} as {}.",
            self.address,
            self.device_id
        );
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("Device {} disconnected.", self.device_id);
    }
}

impl Handler<ActorQueryFrame> for Session {
    type Result = actix::ResponseActFuture<
        Self,
        std::result::Result<ActorStateFrame, DeviceCommunicationError>,
    >;

    fn handle(&mut self, frame: ActorQueryFrame, ctx: &mut Self::Context) -> Self::Result {
        use actix::prelude::*;
        let frame: query::Frame = frame.into();
        let frame = Frame::Query(frame);

        let mut rx = self.state_channel.subscribe();
        let json = match serde_json::to_string(&frame) {
            Ok(json) => json,
            Err(err) => return Box::pin(async move { Err(err.into()) }.into_actor(self)),
        };
        ctx.text(json);

        let fut = async move {
            let resp = tokio::time::timeout(QUERY_TIMEOUT, rx.recv())
                .await
                .map_err(|_| DeviceCommunicationError::Timeout)?
                .unwrap();

            Ok::<ActorStateFrame, DeviceCommunicationError>(resp.into())
        }
        .into_actor(self);

        Box::pin(fut)
    }
}

impl Handler<ActorExecuteFrame> for Session {
    type Result = actix::ResponseActFuture<
        Self,
        std::result::Result<ActorExecuteResponseFrame, DeviceCommunicationError>,
    >;

    fn handle(&mut self, frame: ActorExecuteFrame, ctx: &mut Self::Context) -> Self::Result {
        use actix::prelude::*;
        let frame: execute::Frame = frame.into();
        let request_id = frame.id;
        let frame = Frame::Execute(frame);

        let json = match serde_json::to_string(&frame) {
            Ok(json) => json,
            Err(err) => return Box::pin(async move { Err(err.into()) }.into_actor(self)),
        };

        let (tx, rx) = oneshot::channel();
        self.execute_channels.insert(request_id, tx);
        ctx.text(json);

        let fut = async move {
            let resp = tokio::time::timeout(EXECUTE_TIMEOUT, rx)
                .await
                .map_err(|_| DeviceCommunicationError::Timeout)?
                .expect("Sender is dropped when receiving response");

            Ok::<ActorExecuteResponseFrame, DeviceCommunicationError>(resp.into())
        }
        .into_actor(self)
        .map(move |res, session: &mut Self, _| {
            session.execute_channels.remove(&request_id);
            res
        });

        Box::pin(fut)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(err) => {
                tracing::error!("message error: {}", err);
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Text(text) => {
                // FIXME: handle it properly to avoid mutex poisoning by panicking
                let frame = serde_json::from_str(&text).expect("client sent invalid JSON");
                match frame {
                    Frame::State(frame) => {
                        self.state_channel.send(frame).expect("failed sending");
                    }
                    Frame::Query(_frame) => panic!("Unexpected query received"),
                    Frame::Execute(_) => panic!("Unexpected execute received"),
                    Frame::ExecuteResponse(frame) => {
                        tracing::debug!("Received Execute Response, ID: {:?}", frame.id);
                        self.execute_channels
                            .remove(&frame.id)
                            .expect("no one was waiting for response")
                            .send(frame)
                            .expect("failed sending response");
                    }
                    // FIXME: handle that by returning some kind of response, or at least logging
                    _ => unimplemented!(),
                }
            }
            ws::Message::Binary(bytes) => {
                tracing::info!("Received binary: {:?}", bytes);
            }
            ws::Message::Continuation(item) => {
                tracing::info!("Received continuation: {:?}", item);
            }
            ws::Message::Ping(bytes) => {
                tracing::info!("Received ping: {:?}", bytes);
                ctx.pong(b"");
            }
            ws::Message::Pong(bytes) => {
                tracing::info!("Received pong: {:?}", bytes);
            }
            ws::Message::Close(bytes) => {
                tracing::info!("Connection closed, reason: {:?}", bytes);
            }
            ws::Message::Nop => {
                tracing::info!("Received no operation");
            }
        };
    }
}
