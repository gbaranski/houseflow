use super::Session;
use super::SessionInternals;
use crate::State;
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::Extension;
use axum::extract::TypedHeader;
use axum::response::IntoResponse;
use houseflow_types::device;
use houseflow_types::errors::AuthError;
use houseflow_types::errors::LighthouseError;
use houseflow_types::errors::ServerError;
use std::str::FromStr;
use tracing::Instrument;

pub struct DeviceCredentials(device::ID, device::Password);

fn verify_password(hash: &str, password: &str) -> Result<(), AuthError> {
    match argon2::verify_encoded(hash, password.as_bytes()).unwrap() {
        true => Ok(()),
        false => Err(AuthError::InvalidPassword),
    }
}

#[async_trait]
impl axum::extract::FromRequest<Body> for DeviceCredentials {
    type Rejection = ServerError;

    async fn from_request(
        req: &mut axum::extract::RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(headers::Authorization(authorization)) =
            TypedHeader::<headers::Authorization<headers::authorization::Basic>>::from_request(req)
                .await
                .map_err(|err| AuthError::InvalidAuthorizationHeader(err.to_string()))?;
        let device_id = device::ID::from_str(authorization.username()).map_err(|err| {
            AuthError::InvalidAuthorizationHeader(format!("invalid device id: {}", err))
        })?;

        Ok(Self(device_id, authorization.password().to_owned()))
    }
}

#[tracing::instrument(name = "WebSocket", skip(websocket, state, device_password), err)]
pub async fn handle(
    websocket: WebSocketUpgrade,
    Extension(state): Extension<State>,
    Extension(socket_address): Extension<std::net::SocketAddr>,
    DeviceCredentials(device_id, device_password): DeviceCredentials,
) -> Result<impl IntoResponse, ServerError> {
    let device = state
        .config
        .get_device(&device_id)
        .ok_or(AuthError::DeviceNotFound)?;
    verify_password(device.password_hash.as_ref().unwrap(), &device_password)?;
    if state.sessions.contains_key(&device.id) {
        return Err(LighthouseError::AlreadyConnected.into());
    }

    Ok(websocket.on_upgrade(move |stream|  {
        let span = tracing::span!(tracing::Level::INFO, "WebSocket", address = %socket_address, device_id = %device.id);
        async move {
            let session_internals = SessionInternals::new();
            let session = Session::new(&session_internals);
            tracing::info!("Device connected");
            state.sessions.insert(device.id, session.clone());
            match session.run(stream, session_internals).await {
                Ok(_) => tracing::info!("Connection closed"),
                Err(err) => tracing::error!("Connection closed with error: {}", err),
            }
            state.sessions.remove(&device.id);
        }.instrument(span)
    }))
}
