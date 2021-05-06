use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

use houseflow_types::{DeviceID, DevicePassword};
use itertools::Itertools;
use session::Session;
use std::str::FromStr;

mod channels;
mod session;
mod store;

fn parse_authorization_header(req: &HttpRequest) -> Result<(DeviceID, DevicePassword), String> {
    let header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .ok_or(String::from("`Authorization` header is missing"))?
        .to_str()
        .map_err(|err| format!("Invalid string `Authorization` header, error: `{}`", err))?;

    let mut iter = header.split_whitespace();
    let auth_type = iter
        .next()
        .ok_or("Missing auth type in `Authorization` header")?;
    if auth_type != "Basic" {
        return Err(format!("Invalid auth type: {}", auth_type));
    }
    let credentials = iter
        .next()
        .ok_or("Missing credentials in `Authorization` header")?;

    let (device_id, device_password) = credentials
        .split_terminator(":")
        .take(2)
        .next_tuple()
        .ok_or("Missing ID/Password in `Authorization` header")?;

    Ok((
        DeviceID::from_str(device_id).map_err(|err| err.to_string())?,
        DevicePassword::from_str(device_password).map_err(|err| err.to_string())?,
    ))
}

#[get("/ws")]
async fn on_websocket(req: HttpRequest, stream: web::Payload) -> impl Responder {
    let address = req.peer_addr().unwrap();
    let (device_id, device_password) = match parse_authorization_header(&req) {
        Ok(v) => v,
        Err(err) => return Ok(HttpResponse::BadRequest().body(err)), // TODO: Consider changing Ok to Err
    };
    log::debug!(
        "DeviceID: {}, DevicePassword: {}",
        device_id,
        device_password
    );
    let session = Session::new(device_id, address);
    let response = ws::start(session, &req, stream);
    log::debug!("Response: {:?}", response);
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    // let connection_store = store::Store::new();
    let addr = "127.0.0.1:8080";
    let server = HttpServer::new(move || App::new().service(on_websocket)).bind(&addr)?;

    server.run().await
}
