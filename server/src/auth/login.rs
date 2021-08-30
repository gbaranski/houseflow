use crate::State;
use axum::extract::Extension;
use axum::Json;
use chrono::Duration;
use chrono::Utc;
use houseflow_types::auth::login::Request;
use houseflow_types::auth::login::Response;
use houseflow_types::errors::AuthError;
use houseflow_types::errors::ServerError;
use houseflow_types::token::AccessToken;
use houseflow_types::token::AccessTokenPayload;
use houseflow_types::token::RefreshToken;
use houseflow_types::token::RefreshTokenPayload;
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
        .config
        .get_user_by_email(&request.email)
        .ok_or(AuthError::UserNotFound)?;

    let refresh_token = RefreshToken::new(
        state.config.secrets.refresh_key.as_bytes(),
        RefreshTokenPayload {
            sub: user.id.clone(),
            exp: Some(Utc::now() + Duration::days(7)),
        },
    );
    let access_token = AccessToken::new(
        state.config.secrets.access_key.as_bytes(),
        AccessTokenPayload {
            sub: user.id.clone(),
            exp: Utc::now() + Duration::minutes(10),
        },
    );

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
    use houseflow_types::errors::AuthError;
    use houseflow_types::errors::ServerError;
    use houseflow_types::token::AccessToken;
    use houseflow_types::token::RefreshToken;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn valid() {
        let user = get_user();
        let state = get_state(
            &mpsc::unbounded_channel(),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![user.clone()],
        );
        let Json(response) = super::handle(state.clone(), Json(Request { email: user.email }))
            .await
            .unwrap();
        let (at, rt) = (response.access_token, response.refresh_token);
        let (at, rt) = (
            AccessToken::decode(state.config.secrets.access_key.as_bytes(), &at).unwrap(),
            RefreshToken::decode(state.config.secrets.refresh_key.as_bytes(), &rt).unwrap(),
        );
        assert_eq!(at.sub, rt.sub);
    }

    #[tokio::test]
    async fn invalid_password() {
        let user = get_user();
        let state = get_state(
            &mpsc::unbounded_channel(),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![user.clone()],
        );
        let response = super::handle(state.clone(), Json(Request { email: user.email }))
            .await
            .unwrap_err();

        assert_eq!(response, ServerError::AuthError(AuthError::InvalidPassword));
    }

    #[tokio::test]
    async fn not_existing_user() {
        let user = get_user();
        let state = get_state(
            &mpsc::unbounded_channel(),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        );
        let response = super::handle(state.clone(), Json(Request { email: user.email }))
            .await
            .unwrap_err();

        assert_eq!(response, ServerError::AuthError(AuthError::UserNotFound));
    }
}
