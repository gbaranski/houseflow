use super::{Session, SessionInternals};
use crate::State;
use axum::{
    extract::{ws::WebSocketUpgrade, ConnectInfo, Extension, TypedHeader},
    response::IntoResponse,
};
use houseflow_types::{
    errors::{AuthError, LighthouseError, ServerError},
    DeviceID,
};
use std::str::FromStr;

#[tracing::instrument(
    name = "DeviceConnect",
    skip(websocket, state),
)]
pub async fn handle(
    websocket: WebSocketUpgrade,
    Extension(state): Extension<State>,
    ConnectInfo(socket_address): ConnectInfo<std::net::SocketAddr>,
    TypedHeader(headers::Authorization(authorization)): TypedHeader<
        headers::Authorization<headers::authorization::Basic>,
    >,
) -> Result<impl IntoResponse, ServerError> {
    let device_id = DeviceID::from_str(authorization.username()).map_err(|err| {
        AuthError::InvalidAuthorizationHeader(format!("invalid device id: {}", err))
    })?;
    let device = state
        .config
        .get_device(&device_id)
        .ok_or(AuthError::DeviceNotFound)?;
    crate::verify_password(
        device.password_hash.as_ref().unwrap(),
        authorization.password(),
    )?;
    if state.sessions.lock().unwrap().contains_key(&device.id) {
        return Err(LighthouseError::AlreadyConnected.into());
    }

    use tracing::Instrument;
    let span = tracing::Span::current();
    span.record("device_id", &tracing::field::display(&device.id));
    span.record("device_type", &tracing::field::display(&device.device_type));
    span.record("device_name", &tracing::field::display(&device.name));

    Ok(websocket.on_upgrade(|stream| {
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
    }))
}
