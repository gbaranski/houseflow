use actix_web::{get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use itertools::Itertools;
use lighthouse_proto::{execute, execute_response};
use session::Session;
use std::collections::HashMap;
use std::str::FromStr;
use tokio::sync::Mutex;
use types::{DeviceID, DevicePassword};
pub use config::Config;

mod aliases;
pub mod config;
mod session;

fn parse_authorization_header(req: &HttpRequest) -> Result<(DeviceID, DevicePassword), String> {
    let header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .ok_or_else(|| String::from("`Authorization` header is missing"))?
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
        .split_terminator(':')
        .take(2)
        .next_tuple()
        .ok_or("Missing ID/Password in `Authorization` header")?;

    Ok((
        DeviceID::from_str(device_id).map_err(|err| err.to_string())?,
        DevicePassword::from_str(device_password).map_err(|err| err.to_string())?,
    ))
}

#[get("/ws")]
async fn on_websocket(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let address = req.peer_addr().unwrap();
    let (device_id, device_password) = match parse_authorization_header(&req) {
        Ok(v) => v,
        Err(err) => {
            log::debug!("session init error: {}", err);
            return Ok::<HttpResponse, actix_web::Error>(HttpResponse::BadRequest().body(err));
        } // TODO: Consider changing Ok to Err
    };
    log::debug!(
        "DeviceID: {}, DevicePassword: {}",
        device_id,
        device_password
    );
    let session = Session::new(device_id.clone(), address);
    let (address, response) = ws::start_with_addr(session, &req, stream).unwrap();
    app_state.sessions.lock().await.insert(device_id, address);
    log::debug!("Response: {:?}", response);
    Ok(response)
}

#[post("/execute/{device_id}")]
async fn on_command(
    path: web::Path<String>,
    frame: web::Json<execute::Frame>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let device_id = path.into_inner();
    let device_id = DeviceID::from_str(&device_id)
        .map_err(|err| HttpResponse::BadRequest().body(format!("Invalid DeviceID: {}", err)))?;

    let response: execute_response::Frame = app_state
        .sessions
        .lock()
        .await
        .get(&device_id)
        .ok_or_else(|| HttpResponse::NotFound().body("Device not found"))?
        .send(aliases::ActorExecuteFrame::from(frame.into_inner()))
        .await
        .unwrap()
        .unwrap()
        .into();

    log::debug!("Response: {:?}", response);
    Ok(HttpResponse::Ok().json(response))
}

pub(crate) struct AppState {
    sessions: Mutex<HashMap<DeviceID, actix::Addr<Session>>>,
}

pub(crate) fn config(cfg: &mut web::ServiceConfig, app_state: web::Data<AppState>) {
    cfg.app_data(app_state)
        .service(on_websocket)
        .service(on_command);
}

pub async fn run(config: Config) -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        sessions: Default::default(),
    });

    log::info!("Starting Lighthouse Broker");
    let address = format!("{}:{}", config.host, config.port);
    let server = HttpServer::new(move || {
        App::new()
            .configure(|cfg| crate::config(cfg, app_state.clone()))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(address.clone())?;

    log::info!("Starting HTTP Server at `{}`", address);

    server.run().await?;

    Ok(())
}
