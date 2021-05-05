use actix::prelude::*;
use actix::{Actor, Handler, StreamHandler};
use actix_web_actors::ws;
use houseflow_types::ClientID;
use lighthouse_api::Request;
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionError {
    // #[error("websocket error: {0}")]
    // WebsocketError(#[from] warp::Error),
}

pub struct Session {
    client_id: ClientID,
    address: SocketAddr,
}

impl Session {
    pub fn new(client_id: ClientID, address: SocketAddr) -> Self {
        Self { client_id, address }
    }
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("New client connected from {} as {}.", self.address, self.client_id);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        log::info!("Client {} disconnected.", self.client_id);
    }
}

impl Handler<Request> for Session {
    type Result = ();

    fn handle(&mut self, request: Request, ctx: &mut Self::Context) {
        log::info!("Received request for {}", self.client_id);
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
