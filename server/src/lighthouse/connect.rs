use super::Session;
use super::SessionInternals;
use crate::State;
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::Extension;
use axum::extract::TypedHeader;
use axum::response::IntoResponse;
use houseflow_types::errors::AuthError;
use houseflow_types::errors::LighthouseError;
use houseflow_types::errors::ServerError;
use houseflow_types::hub;
use tracing::Instrument;

pub struct HubCredentials(hub::ID, hub::Password);

fn verify_password(hash: &str, password: &str) -> Result<(), AuthError> {
    match argon2::verify_encoded(hash, password.as_bytes()).unwrap() {
        true => Ok(()),
        false => Err(AuthError::InvalidPassword),
    }
}

#[async_trait]
impl axum::extract::FromRequest<Body> for HubCredentials {
    type Rejection = ServerError;

    async fn from_request(
        req: &mut axum::extract::RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(headers::Authorization(authorization)) =
            TypedHeader::<headers::Authorization<headers::authorization::Basic>>::from_request(req)
                .await
                .map_err(|err| AuthError::InvalidAuthorizationHeader(err.to_string()))?;
        let hub_id = hub::ID::parse_str(authorization.username()).map_err(|err| {
            AuthError::InvalidAuthorizationHeader(format!("invalid hub id: {}", err))
        })?;

        Ok(Self(hub_id, authorization.password().to_owned()))
    }
}

#[tracing::instrument(name = "WebSocket", skip(websocket, state, hub_password), err)]
pub async fn handle(
    websocket: WebSocketUpgrade,
    Extension(state): Extension<State>,
    Extension(socket_address): Extension<std::net::SocketAddr>,
    HubCredentials(hub_id, hub_password): HubCredentials,
) -> Result<impl IntoResponse, ServerError> {
    let hub = state
        .config
        .get_hub(&hub_id)
        .ok_or(AuthError::HubNotFound)?
        .to_owned();
    verify_password(&hub.password_hash, &hub_password)?;
    if state.sessions.contains_key(&hub.id) {
        return Err(LighthouseError::AlreadyConnected.into());
    }

    Ok(websocket.on_upgrade(move |stream|  {
        let span = tracing::span!(tracing::Level::INFO, "WebSocket", address = %socket_address, hub_id = %hub.id);
        async move {
            let session_internals = SessionInternals::new();
            let session = Session::new(&session_internals);
            tracing::info!("Hub connected");
            state.sessions.insert(hub.id, session.clone());
            match session.run(stream, session_internals).await {
                Ok(_) => tracing::info!("Connection closed"),
                Err(err) => tracing::error!("Connection closed with error: {}", err),
            }
            state.sessions.remove(&hub.id);
        }.instrument(span)
    }))
}
