use crate::{extractors::RefreshToken, State};
use axum::{extract, response};
use houseflow_types::{auth::logout::Response, errors::ServerError};
use tracing::Level;

#[tracing::instrument(name = "Logout", skip(state, refresh_token), err)]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    RefreshToken(refresh_token): RefreshToken,
) -> Result<response::Json<Response>, ServerError> {
    state.token_store.remove(&refresh_token.tid).await.unwrap();
    tracing::event!(Level::INFO, user_id = %refresh_token.sub, "Logged out");
    Ok(response::Json(Response {}))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

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
        state.token_store.add(&refresh_token.tid, refresh_token.exp.as_ref()).await.unwrap();
        let _ = super::handle(
            state.clone(),
            crate::extractors::RefreshToken(refresh_token.clone()),
        )
        .await
        .unwrap();
        assert!(!state.token_store.exists(&refresh_token.tid).await.unwrap());
    }
}
