use crate::aliases::{
    ActorCommandFrame, ActorCommandResponseFrame, ActorStateCheckFrame, ActorStateFrame,
};
use actix::{Actor, ActorContext, Handler, StreamHandler};
use actix_web_actors::ws;
use bytes::BytesMut;
use types::DeviceID;
use lighthouse_proto::{
    command, command_response, state, state_check, Decoder, Encoder, Frame, FrameID,
};
use lighthouse_types::DeviceError;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::{broadcast, oneshot};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
const STATE_CHANNEL_SIZE: usize = 4;

#[derive(Debug, Error)]
pub enum SessionError {
    // #[error("websocket error: {0}")]
// WebsocketError(#[from] warp::Error),
}

pub struct Session {
    device_id: DeviceID,
    address: SocketAddr,
    pub command_channels: HashMap<FrameID, oneshot::Sender<command_response::Frame>>,
    pub state_channel: broadcast::Sender<state::Frame>,
}

impl Session {
    pub fn new(device_id: DeviceID, address: SocketAddr) -> Self {
        let (state_channel, _) = broadcast::channel(STATE_CHANNEL_SIZE);

        Self {
            device_id,
            address,
            state_channel,
            command_channels: Default::default(),
        }
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!(
            "New device connected from {} as {}.",
            self.address,
            self.device_id
        );
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Device {} disconnected.", self.device_id);
    }
}

impl Handler<ActorStateCheckFrame> for Session {
    type Result = actix::ResponseActFuture<Self, std::result::Result<ActorStateFrame, DeviceError>>;

    fn handle(&mut self, frame: ActorStateCheckFrame, ctx: &mut Self::Context) -> Self::Result {
        use actix::prelude::*;
        let frame: state_check::Frame = frame.into();

        let mut buf = BytesMut::with_capacity(512);
        let mut rx = self.state_channel.subscribe();
        frame.encode(&mut buf);
        ctx.binary(buf);

        let fut = async move {
            let resp = tokio::time::timeout(REQUEST_TIMEOUT, rx.recv())
                .await
                .map_err(|_| DeviceError::Timeout)?
                .expect("Sender is dropped when receiving response");

            Ok::<ActorStateFrame, DeviceError>(resp.into())
        }
        .into_actor(self);

        Box::pin(fut)
    }
}

impl Handler<ActorCommandFrame> for Session {
    type Result =
        actix::ResponseActFuture<Self, std::result::Result<ActorCommandResponseFrame, DeviceError>>;

    fn handle(&mut self, frame: ActorCommandFrame, ctx: &mut Self::Context) -> Self::Result {
        use actix::prelude::*;
        let frame: command::Frame = frame.into();

        let mut buf = BytesMut::with_capacity(512);
        let (tx, rx) = oneshot::channel();
        let request_id = frame.id.clone();
        self.command_channels.insert(request_id.clone(), tx);
        frame.encode(&mut buf);
        ctx.binary(buf);

        let fut = async move {
            let resp = tokio::time::timeout(REQUEST_TIMEOUT, rx)
                .await
                .map_err(|_| DeviceError::Timeout)?
                .expect("Sender is dropped when receiving response");

            Ok::<ActorCommandResponseFrame, DeviceError>(resp.into())
        }
        .into_actor(self)
        .map(move |res, session: &mut Self, _| {
            session.command_channels.remove(&request_id);
            res
        });

        Box::pin(fut)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        log::debug!("Handling msg");
        let msg = match msg {
            Ok(msg) => msg,
            Err(err) => {
                ctx.stop();
                log::error!("Error occured: {}", err);
                return;
            }
        };

        match msg {
            ws::Message::Text(text) => {
                log::info!("Received text: {}", text);
            }
            ws::Message::Binary(mut bytes) => {
                let frame = Frame::decode(&mut bytes).expect("failed decoding");
                match frame {
                    Frame::NoOperation(_frame) => (),
                    Frame::State(frame) => {
                        self.state_channel.send(frame).expect("failed sending");
                    }
                    Frame::StateCheck(_frame) => panic!("Unexpected state check received"),
                    Frame::Command(_) => panic!("Unexpected command received"),
                    Frame::CommandResponse(frame) => {
                        log::debug!("Received CommandResponse, ID: {:?}", frame.id);
                        self.command_channels
                            .remove(&frame.id)
                            .expect("no one was waiting for response")
                            .send(frame)
                            .expect("failed sending response");
                    }
                }
            }
            ws::Message::Continuation(item) => {
                log::info!("Received continuation: {:?}", item);
            }
            ws::Message::Ping(bytes) => {
                log::info!("Received ping: {:?}", bytes);
            }
            ws::Message::Pong(bytes) => {
                log::info!("Received pong: {:?}", bytes);
            }
            ws::Message::Close(bytes) => {
                log::info!("Connection closed, reason: {:?}", bytes);
            }
            ws::Message::Nop => {
                log::info!("Received no operation");
            }
        };
    }
}
