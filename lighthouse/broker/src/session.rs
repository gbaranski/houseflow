use actix::{Actor, ActorContext, Handler, StreamHandler};
use actix_web_actors::ws;
use bytes::BytesMut;
use houseflow_types::DeviceID;
use lighthouse_api::{Request, RequestError, Response};
use lighthouse_proto::{Decoder, Encoder, Frame, FrameID};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::oneshot;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Error)]
pub enum SessionError {
    // #[error("websocket error: {0}")]
// WebsocketError(#[from] warp::Error),
}

pub struct Session {
    device_id: DeviceID,
    address: SocketAddr,
    pub response_channels: HashMap<FrameID, oneshot::Sender<Response>>,
}

impl Session {
    pub fn new(device_id: DeviceID, address: SocketAddr) -> Self {
        Self {
            device_id,
            address,
            response_channels: HashMap::new(),
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

impl Handler<Request> for Session {
    type Result = actix::ResponseActFuture<Self, std::result::Result<Response, RequestError>>;

    fn handle(&mut self, request: Request, ctx: &mut Self::Context) -> Self::Result {
        use actix::prelude::*;

        let mut buf = BytesMut::with_capacity(512);
        let (tx, rx) = oneshot::channel();
        let request_id = request.id();
        let frame: Frame = request.into();
        self.response_channels.insert(request_id, tx);
        frame.encode(&mut buf);
        ctx.binary(buf);

        let fut = async move {
            let resp = tokio::time::timeout(REQUEST_TIMEOUT, rx)
                .await
                .map_err(|_| RequestError::Timeout)?
                .expect("Sender is dropped when receiving response");

            Ok::<_, RequestError>(resp)
        }
        .into_actor(self)
        .map(move |res, session: &mut Self, _| {
            session.response_channels.remove(&request_id);
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
                    Frame::NoOperation => return,
                    Frame::Execute(_) => panic!("Unexpected execute received"),
                    Frame::ExecuteResponse(frame) => {
                        log::debug!("Received ExecuteResponse, ID: {}", frame.id);
                        self.response_channels
                            .remove(&frame.id)
                            .expect("no one was waiting for response")
                            .send(Response::Execute(frame))
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
