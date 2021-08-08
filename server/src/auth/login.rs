use crate::State;
use axum::{extract::Extension, Json};
use chrono::{Duration, Utc};
use houseflow_types::{
    auth::login::{Request, Response},
    errors::{AuthError, ServerError},
    token::{AccessToken, AccessTokenPayload, RefreshToken, RefreshTokenPayload},
};
use tracing::Level;

#[tracing::instrument(
    name = "Login",
    skip(state, request),
    fields(
        email = %request.email,
    ),
    err,
)]
pub async fn handle(
    Extension(state): Extension<State>,
    Json(request): Json<Request>,
) -> Result<Json<Response>, ServerError> {
    validator::Validate::validate(&request)?;
    let user = state
        .database
        .get_user_by_email(&request.email)?
        .ok_or(AuthError::UserNotFound)?;

    crate::verify_password(&user.password_hash, &request.password)?;
    let refresh_token = RefreshToken::new(
        state.config.secrets.refresh_key.as_bytes(),
        RefreshTokenPayload {
            sub: user.id.clone(),
            exp: Some(Utc::now() + Duration::days(7)),
            tid: rand::random(),
        },
    );
    let access_token = AccessToken::new(
        state.config.secrets.access_key.as_bytes(),
        AccessTokenPayload {
            sub: user.id.clone(),
            exp: Utc::now() + Duration::minutes(10),
        },
    );
    state
        .token_store
        .add(&refresh_token.tid, refresh_token.exp.as_ref())
        .await?;

    tracing::event!(Level::INFO, user_id = %user.id, "Logged in");

    Ok(Json(Response {
        access_token: access_token.encode(),
        refresh_token: refresh_token.encode(),
    }))
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test_utils::*;
    use axum::Json;
    use houseflow_types::{
        errors::{AuthError, ServerError},
        token::{AccessToken, RefreshToken},
    };

    #[tokio::test]
    async fn valid() {
        let state = get_state();
        let user = get_user();
        state.database.add_user(&user).unwrap();
        let Json(response) = super::handle(
            state.clone(),
            Json(Request {
                email: user.email,
                password: PASSWORD.into(),
            }),
        )
        .await
        .unwrap();
        let (at, rt) = (response.access_token, response.refresh_token);
        let (at, rt) = (
            AccessToken::decode(state.config.secrets.access_key.as_bytes(), &at).unwrap(),
            RefreshToken::decode(state.config.secrets.refresh_key.as_bytes(), &rt).unwrap(),
        );
        assert_eq!(at.sub, rt.sub);
        assert!(
            state.token_store.exists(&rt.tid).await.unwrap(),
            "refresh token not found in token store"
        );
    }

    #[tokio::test]
    async fn invalid_password() {
        let state = get_state();
        let user = get_user();
        state.database.add_user(&user).unwrap();
        let response = super::handle(
            state.clone(),
            Json(Request {
                email: user.email,
                password: PASSWORD_INVALID.into(),
            }),
        )
        .await
        .unwrap_err();

        assert_eq!(response, ServerError::AuthError(AuthError::InvalidPassword));
    }

    #[tokio::test]
    async fn not_existing_user() {
        let state = get_state();
        let user = get_user();
        let response = super::handle(
            state.clone(),
            Json(Request {
                email: user.email,
                password: PASSWORD.into(),
            }),
        )
        .await
        .unwrap_err();

        assert_eq!(response, ServerError::AuthError(AuthError::UserNotFound));
    }
}
