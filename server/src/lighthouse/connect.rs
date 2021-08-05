use super::{Session, SessionInternals};
use crate:: State;
use async_trait::async_trait;
use axum::extract;
use houseflow_types::{errors::{AuthError, ServerError, LighthouseError}, DeviceID};
use std::str::FromStr;

pub struct WebsocketDevice(pub houseflow_types::Device);

#[async_trait]
impl axum::extract::FromRequest<axum::body::Body> for WebsocketDevice {
    type Rejection = ServerError;

    async fn from_request(
        req: &mut extract::RequestParts<axum::body::Body>,
    ) -> Result<Self, Self::Rejection> {
        let axum::extract::TypedHeader(headers::Authorization(header)) = axum::extract::TypedHeader::<
            headers::Authorization<headers::authorization::Basic>,
        >::from_request(req)
        .await
        .map_err(|_| AuthError::InvalidAuthorizationHeader(String::from("bad header")))?;
        let state: State = req.extensions().unwrap().get::<State>().unwrap().clone();
        let device_id = DeviceID::from_str(header.username()).map_err(|err| {
            AuthError::InvalidAuthorizationHeader(format!("invalid device id: {}", err))
        })?;
        let device = state
            .database
            .get_device(&device_id)?
            .ok_or(AuthError::DeviceNotFound)?;
        crate::verify_password(device.password_hash.as_ref().unwrap(), header.password())?;
        if state.sessions.lock().unwrap().contains_key(&device.id) {
            return Err(LighthouseError::AlreadyConnected.into());
        }
        Ok(Self(device))
    }
}

#[tracing::instrument(name = "DeviceConnect", skip(state, stream))]
pub async fn on_websocket(
    stream: axum::ws::WebSocket,
    extract::Extension(state): extract::Extension<State>,
    extract::ConnectInfo(socket_address): extract::ConnectInfo<std::net::SocketAddr>,
    WebsocketDevice(device): WebsocketDevice,
) {
    let session_internals = SessionInternals::new();
    let session = Session::new(&session_internals);
    tracing::info!("Device connected");
    state
        .sessions
        .lock()
        .unwrap()
        .insert(device.id.clone(), session.clone());
    match session.run(stream, session_internals).await {
        Ok(_) => tracing::info!("Connection closed"),
        Err(err) => tracing::error!("Connection closed with error: {}", err),
    }
    state.sessions.lock().unwrap().remove(&device.id);
}
