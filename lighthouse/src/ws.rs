use actix::prelude::*;
use actix_web::{HttpRequest, HttpResponse, web, get};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct Websockets {
    last_heartbeat: Instant,
}

impl Websockets {
    pub fn new() -> Self {
        Self{
            last_heartbeat: Instant::now(),
        }
    }
}

impl Actor for Websockets {
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
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Websockets {
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
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}


#[get("/ws")]
pub async fn index(
    request: HttpRequest,
    stream: web::Payload,
) -> actix_web::Result<HttpResponse> {
    let res = ws::start(Websockets::new(), &request, stream);
    res
}
