use actix::prelude::*;
use actix::dev::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message)]
#[rtype(result = "crate::Response")]
pub struct ExecuteRequest {
    pub params: std::collections::HashMap<String, String>,
    pub command: String,
}


impl<A, M> MessageResponse<A, M> for crate::Response
where
    A: Actor,
    M: Message<Result = crate::Response>,
{
    fn handle(self, _: &mut A::Context, tx: Option<OneshotSender<crate::Response>>) {
        if let Some(tx) = tx {
            if !tx.send(self).is_ok() {
                log::error!("fail sending message response");
            }
        }
    }
}


pub struct WebsocketSession {
    last_heartbeat: Instant,
}

impl WebsocketSession {
    pub fn new() -> Self {
        Self{
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

/// Handler for `ws::Message`
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
            Ok(ws::Message::Text(text)) => ctx.text(format!("Response: {}", text)),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl Handler<ExecuteRequest> for WebsocketSession {
    type Result = crate::Response;

    fn handle(
        &mut self, 
        req: ExecuteRequest, 
        ctx: &mut Self::Context
    ) -> Self::Result {
        ctx.text("Hello world");

        Self::Result {
            status: crate::ResponseStatus::Success,
            states: std::collections::HashMap::new(),
            error_code: None,
        }

    }
}
