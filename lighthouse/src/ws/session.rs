use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use tokio::sync::oneshot;
use uuid::Uuid;
use std::collections::HashMap;
use crate::{ExecuteResponse, ExecuteResponseStatus};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebsocketSession {
    last_heartbeat: Instant,

    pub response_channels: Vec<(Uuid, Option<oneshot::Sender<ExecuteResponse>>)>,
}

impl WebsocketSession {
    pub fn new() -> Self {
        Self{
            response_channels: vec![],
            last_heartbeat: Instant::now(),
        }
    }
}

impl Actor for WebsocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            let expired = Instant::now().duration_since(act.last_heartbeat) > CLIENT_TIMEOUT;

            if expired {
                log::info!("Websocket client disconnected because ping has not been received");
                ctx.stop();
            } else {
                ctx.ping(&[]);
            }
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let ch = &mut self.response_channels[0]; // temporary constant
                let tx = ch.1.take().unwrap();
                let resp = ExecuteResponse{
                    status: ExecuteResponseStatus::Error,
                    states: HashMap::new(),
                    error_code: Some("dsad".to_string())
                };
                if let Err(_) = tx.send(resp) {
                    println!("the receiver has dropped");
                }

                ctx.text(format!("Response: {}", text));
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
