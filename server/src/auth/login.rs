use crate::{token_store::Error as TokenStoreError, TokenStore};
use actix_web::web::{Data, Json};
use chrono::{Duration, Utc};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    auth::login::{Request, ResponseBody, ResponseError},
    token::{AccessToken, AccessTokenPayload, RefreshToken, RefreshTokenPayload},
};
use tracing::Level;

fn verify_password(hash: &str, password: &str) -> Result<(), ResponseError> {
    match argon2::verify_encoded(hash, password.as_bytes()).unwrap() {
        true => Ok(()),
        false => Err(ResponseError::InvalidPassword),
    }
}

#[tracing::instrument(
    name = "Login",
    skip(request, token_store, config, db),
    fields(
        email = %request.email,
    ),
)]
pub async fn on_login(
    Json(request): Json<Request>,
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
    db: Data<dyn Database>,
) -> Result<Json<ResponseBody>, ResponseError> {
    validator::Validate::validate(&request).map_err(houseflow_types::ValidationError::from)?;
    let user = db
        .get_user_by_email(&request.email)
        .map_err(houseflow_db::Error::into_internal_server_error)?
        .ok_or(ResponseError::UserNotFound)?;

    verify_password(&user.password_hash, &request.password)?;
    let refresh_token = RefreshToken::new(
        config.secrets.refresh_key.as_bytes(),
        RefreshTokenPayload {
            sub: user.id.clone(),
            exp: Some(Utc::now() + Duration::days(7)), // TODO: extend the time only for Google Actions
            tid: rand::random(),
        },
    );
    let access_token = AccessToken::new(
        config.secrets.access_key.as_bytes(),
        AccessTokenPayload {
            sub: user.id.clone(),
            exp: Utc::now() + Duration::minutes(10),
        },
    );
    token_store
        .add(&refresh_token.tid, refresh_token.exp.as_ref())
        .await
        .map_err(TokenStoreError::into_internal_server_error)?;

    tracing::event!(Level::INFO, user_id = %user.id);

    Ok(Json(ResponseBody {
        access_token: access_token.encode(),
        refresh_token: refresh_token.encode(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[actix_rt::test]
    async fn valid() {
        let state = get_state();
        let user = get_user();
        state.database.add_user(&user).unwrap();
        let response = on_login(
            Json(Request {
                email: user.email,
                password: PASSWORD.into(),
            }),
            state.token_store.clone(),
            state.config.clone(),
            state.database,
        )
        .await
        .unwrap()
        .into_inner();

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

    #[actix_rt::test]
    async fn invalid_password() {
        let state = get_state();
        let user = get_user();
        state.database.add_user(&user).unwrap();
        let response = on_login(
            Json(Request {
                email: user.email,
                password: PASSWORD_INVALID.into(),
            }),
            state.token_store.clone(),
            state.config.clone(),
            state.database,
        )
        .await
        .unwrap_err();

        assert_eq!(response, ResponseError::InvalidPassword);
    }

    #[actix_rt::test]
    async fn not_existing_user() {
        let state = get_state();
        let user = get_user();
        let response = on_login(
            Json(Request {
                email: user.email,
                password: PASSWORD.into(),
            }),
            state.token_store.clone(),
            state.config.clone(),
            state.database,
        )
        .await
        .unwrap_err();

        assert_eq!(response, ResponseError::UserNotFound);
    }
}
