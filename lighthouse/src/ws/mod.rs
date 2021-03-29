use actix_web::{get, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;

mod session;
mod server;

pub use session::{WebsocketSession, ExecuteRequest};

#[get("/ws")]
pub async fn index(
    request: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<super::AppState>,
) -> actix_web::Result<HttpResponse> {
    println!("Received new connection at /ws");
    let id = uuid::Uuid::new_v4().to_string();
    let session = WebsocketSession::new();
    let (addr, res) = ws::start_with_addr(session, &request, stream)?;

    let mut sessions = app_state.sessions.lock().unwrap();
    sessions.insert(id.clone(), addr);
    if sessions.contains_key(&id.clone()) != true {
        println!("failed");
    };

    println!("Websocket session started with ID: {}", id);

    Ok(res)
}
