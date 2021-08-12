use crate::{extractors::RefreshToken, State};
use axum::{extract::Extension, Json};
use houseflow_types::{
    auth::logout::{Request, Response},
    errors::ServerError,
};
use tracing::Level;

#[tracing::instrument(name = "Logout", skip(state, refresh_token, _request), err)]
pub async fn handle(
    Extension(state): Extension<State>,
    RefreshToken(refresh_token): RefreshToken,
    Json(_request): Json<Request>,
) -> Result<Json<Response>, ServerError> {
    state
        .token_blacklist
        .add(&refresh_token.tid, refresh_token.exp)
        .await
        .unwrap();
    tracing::event!(Level::INFO, user_id = %refresh_token.sub, "Logged out");
    Ok(Json(Response {}))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use axum::Json;

    #[tokio::test]
    async fn valid() {
        let state = get_state();
        let user = get_user();
        let refresh_token = houseflow_types::token::RefreshToken::new(
            state.config.secrets.refresh_key.as_bytes(),
            houseflow_types::token::RefreshTokenPayload {
                tid: rand::random(),
                sub: user.id.clone(),
                exp: None,
            },
        );
        state.database.add_user(&user).unwrap();
        let _ = super::handle(
            state.clone(),
            crate::extractors::RefreshToken(refresh_token.clone()),
            Json(super::Request {}),
        )
        .await
        .unwrap();
        assert!(state
            .token_blacklist
            .exists(&refresh_token.tid)
            .await
            .unwrap());
    }
}
