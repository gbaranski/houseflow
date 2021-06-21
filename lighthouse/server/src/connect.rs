use crate::{AppState, Session};
use actix_web::{
    get, http,
    web,
    HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use db::Database;
use itertools::Itertools;
use lighthouse_types::ConnectResponseError;
use std::str::FromStr;
use types::{DeviceID, DevicePassword};

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
pub async fn on_websocket(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<AppState>,
    database: web::Data<dyn Database>,
) -> Result<HttpResponse, ConnectResponseError> {
    let address = req.peer_addr().unwrap();
    let (device_id, device_password) = parse_authorization_header(&req)
        .map_err(ConnectResponseError::InvalidAuthorizationHeader)?;

    let device = database
        .get_device(&device_id)
        .await
        .map_err(|err| ConnectResponseError::InternalError(err.to_string()))?
        .ok_or(ConnectResponseError::InvalidCredentials)?;
    
    if !argon2::verify_encoded(&device.password_hash, device_password.as_bytes()).unwrap() {
        return Err(ConnectResponseError::InvalidCredentials);
    }

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
