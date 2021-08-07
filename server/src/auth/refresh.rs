use crate::{extractors::RefreshToken, State};
use axum::{extract, response};
use chrono::{Duration, Utc};
use houseflow_types::{
    auth::token::{Request, Response},
    errors::{AuthError, ServerError},
    token::{AccessToken, AccessTokenPayload},
};
use tracing::Level;

#[tracing::instrument(name = "Refresh token", skip(state, _request), err)]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    RefreshToken(refresh_token): RefreshToken,
    extract::Json(_request): extract::Json<Request>,
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

    tracing::event!(Level::INFO, user_id = %refresh_token.sub, "Refreshed token");

    Ok(response::Json(Response {
        refresh_token: None,
        access_token: access_token.to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use axum::{extract, response};

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
        state
            .token_store
            .add(&refresh_token.tid, refresh_token.exp.as_ref())
            .await
            .unwrap();
        let response::Json(response) = super::handle(
            state.clone(),
            crate::extractors::RefreshToken(refresh_token.clone()),
            extract::Json(super::Request {}),
        )
        .await
        .unwrap();
        let access_token = houseflow_types::token::AccessToken::decode(
            state.config.secrets.access_key.as_bytes(),
            &response.access_token,
        ).unwrap();
        assert_eq!(access_token.sub, refresh_token.sub);
    }
}
