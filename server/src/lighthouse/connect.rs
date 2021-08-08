use super::{Session, SessionInternals};
use crate::State;
use async_trait::async_trait;
use axum::{
    extract::{ws::WebSocketUpgrade, ConnectInfo, Extension, RequestParts, TypedHeader},
    response::IntoResponse,
};
use houseflow_types::{
    errors::{AuthError, LighthouseError, ServerError},
    DeviceID,
};
use std::str::FromStr;

pub struct WebsocketDevice(pub houseflow_types::Device);

#[async_trait]
impl axum::extract::FromRequest<axum::body::Body> for WebsocketDevice {
    type Rejection = ServerError;

    #[tracing::instrument(err)]
    async fn from_request(
        req: &mut RequestParts<axum::body::Body>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(headers::Authorization(header)) =
            TypedHeader::<headers::Authorization<headers::authorization::Basic>>::from_request(req)
                .await
                .map_err(|err| AuthError::InvalidAuthorizationHeader(err.to_string()))?;
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

#[tracing::instrument(
    name = "DeviceConnect",
    skip(websocket, state, device),
    fields(
        device_id = %device.id,
        device_type = %device.device_type,
        device_name = %device.name
    )
)]
pub async fn handle(
    websocket: WebSocketUpgrade,
    Extension(state): Extension<State>,
    ConnectInfo(socket_address): ConnectInfo<std::net::SocketAddr>,
    WebsocketDevice(device): WebsocketDevice,
) -> impl IntoResponse {
    use tracing::Instrument;
    let span = tracing::Span::current();
    websocket.on_upgrade(|stream| {
        async move {
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
        .instrument(span)
    })
}
