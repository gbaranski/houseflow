use actix::prelude::*;
use actix_web::{get, http, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

use houseflow_types::{DeviceID, DevicePassword};
use session::Session;
use std::convert::TryFrom;

mod channels;
mod session;
mod store;

fn parse_authorization_header(req: &HttpRequest) -> Result<(DeviceID, DevicePassword), String> {
    const VALUE_OFFSET: usize = "Basic ".len();
    const VALUE_SIZE: usize =
        VALUE_OFFSET + format!("{}:{}", DeviceID::default(), DevicePassword::default()).len();

    let header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .ok_or(String::from("`Authorization` header is missing"))?
        .to_str()
        .map_err(|err| format!("Invalid string `Authorization` header, error: `{}`", err))?;

    if header.len() != VALUE_SIZE {
        Err(String::from("`Authorization` header invalid size"))
    } else {
        let id: String = header.chars().take(DeviceID::SIZE).collect();
        let password: String = header.chars().take(DevicePassword::SIZE).collect();
        Ok((
            DeviceID::try_from(id).map_err(|err| err.to_string())?,
            DevicePassword::try_from(password).map_err(|err| err.to_string())?,
        ))
    }
}

#[get("/")]
async fn on_websocket(req: HttpRequest, stream: web::Payload) -> impl Responder {
    // let (type, credentials) = (.
    let address = req.peer_addr().unwrap();
    let (device_id, device_password) = match parse_authorization_header(&req) {
        Ok(v) => v,
        Err(err) => return Ok(HttpResponse::BadRequest().body(err)), // TODO: Consider changing Ok to Err
    };
    let session = Session::new(device_id, address);
    ws::start(session, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let connection_store = store::Store::new();
    let server = HttpServer::new(move || App::new().service(on_websocket));
    //     let store_filter = warp::any().map(move || connection_store.clone());
    //
    //     let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    //     let websocket_path = warp::ws()
    //         .and(warp::path("websocket"))
    //         .and(warp::addr::remote())
    //         .and(store_filter)
    //         .and(warp::header::<ClientID>("client_id"))
    //         .map(
    //             |ws: warp::ws::Ws,
    //              address: Option<SocketAddr>,
    //              store: store::Store,
    //              client_id: ClientID| {
    //                 ws.on_upgrade(move |ws: warp::ws::WebSocket| async move {
    //                     let (request_receiver, request_sender) = channels::new_request_channel();
    //                     let (response_receiver, response_sender) = channels::new_response_channel();
    //                     store
    //                         .add(client_id, (response_receiver, request_sender.clone()))
    //                         .await;
    //                     let session = Session::new(client_id);
    //                     log::info!(
    //                         "New client connected from {} as {}",
    //                         address.unwrap(),
    //                         client_id
    //                     );
    //                     match session
    //                         .run(ws, request_receiver, request_sender, response_sender)
    //                         .await
    //                     {
    //                         Ok(()) => log::info!("Client {} disconnected.", client_id),
    //                         Err(err) => {
    //                             log::error!("Client {} disconnected, error: {}", client_id, err)
    //                         }
    //                     }
    //                     store.remove(&client_id).await;
    //                 })
    //             },
    //         );
    //
    //     warp::serve(websocket_path).run(address).await;
    Ok(())
}
