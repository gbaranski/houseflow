use crate::{extractors::RefreshToken, State};
use axum::{extract, response};
use chrono::{Duration, Utc};
use houseflow_types::{
    auth::token::{Request, Response},
    token::{AccessToken, AccessTokenPayload},
    errors::{ServerError, AuthError},
};
use tracing::Level;

#[tracing::instrument(name = "Refresh token", skip(state))]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    extract::Json(request): extract::Json<Request>,
    RefreshToken(refresh_token): RefreshToken,
) -> Result<response::Json<Response>, ServerError> {
    if !state.token_store.exists(&refresh_token.tid).await? {
        return Err(AuthError::RefreshTokenNotInStore.into());
    }

    let access_token_payload = AccessTokenPayload {
        sub: refresh_token.sub.clone(),
        exp: Utc::now() + Duration::minutes(10),
    };
    let access_token = AccessToken::new(
        state.config.secrets.access_key.as_bytes(),
        access_token_payload,
    );

    tracing::event!(Level::INFO, user_id = %refresh_token.sub);

    Ok(response::Json(Response {
        refresh_token: None,
        access_token: access_token.to_string(),
    }))
}
