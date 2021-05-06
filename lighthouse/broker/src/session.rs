use actix::prelude::*;
use actix::{Actor, Handler, StreamHandler};
use actix_web_actors::ws;
use houseflow_types::DeviceID;
use lighthouse_api::Request;
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionError {
    // #[error("websocket error: {0}")]
    // WebsocketError(#[from] warp::Error),
}

pub struct Session {
    device_id: DeviceID,
    address: SocketAddr,
}

impl Session {
    pub fn new(device_id: DeviceID, address: SocketAddr) -> Self {
        Self { device_id, address }
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("New device connected from {} as {}.", self.address, self.device_id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Device {} disconnected.", self.device_id);
    }
}

impl Handler<Request> for Session {
    type Result = ();

    fn handle(&mut self, _request: Request, _ctx: &mut Self::Context) {
        log::info!("Received request for {}", self.device_id);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
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
            ws::Message::Binary(bytes) => {
                log::info!("Received binary: {:?}", bytes);
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
