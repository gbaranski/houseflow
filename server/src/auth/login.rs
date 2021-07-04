use crate::{token_store::Error as TokenStoreError, TokenStore};
use actix_web::{
    post,
    web::{Data, Json},
};
use chrono::{Duration, Utc};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    auth::login::{Request, ResponseBody, ResponseError},
    token::{AccessToken, AccessTokenPayload, RefreshToken, RefreshTokenPayload},
};

fn verify_password(hash: &str, password: &str) -> Result<(), ResponseError> {
    match argon2::verify_encoded(hash, password.as_bytes()).unwrap() {
        true => Ok(()),
        false => Err(ResponseError::InvalidPassword),
    }
}

#[post("/login")]
pub async fn on_login(
    Json(request): Json<Request>,
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
    db: Data<dyn Database>,
) -> Result<Json<ResponseBody>, ResponseError> {
    validator::Validate::validate(&request).map_err(houseflow_types::ValidationError::from)?;
    let user = db
        .get_user_by_email(&request.email)
        .await
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
            sub: user.id,
            exp: Utc::now() + Duration::minutes(10),
        },
    );
    token_store
        .add(&refresh_token.tid)
        .await
        .map_err(TokenStoreError::into_internal_server_error)?;

    Ok(Json(ResponseBody {
        access_token: access_token.encode(),
        refresh_token: refresh_token.encode(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use actix_web::test;
    use houseflow_types::User;

    use rand::random;

    #[actix_rt::test]
    async fn test_login() {
        let state = get_state();
        let user = User {
            id: random(),
            username: String::from("John Smith"),
            email: String::from("john_smith@example.com"),
            password_hash: PASSWORD_HASH.into(),
        };
        state.database.add_user(&user).await.unwrap();

        let request_body = Request {
            email: user.email,
            password: PASSWORD.into(),
        };
        let request = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&request_body);
        let response = send_request_with_state::<ResponseBody>(request, &state).await;
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
    async fn test_login_invalid_password() {
        let state = get_state();
        let user = User {
            id: random(),
            username: String::from("John Smith"),
            email: String::from("john_smith@example.com"),
            password_hash: PASSWORD_HASH.into(),
        };
        state.database.add_user(&user).await.unwrap();

        let request_body = Request {
            email: user.email,
            password: PASSWORD_INVALID.into(),
        };
        let request = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&request_body);
        let response = send_request_with_state::<ResponseError>(request, &state).await;
        assert_eq!(response, ResponseError::InvalidPassword);
    }

    #[actix_rt::test]
    async fn test_login_not_existing_user() {
        let request_body = Request {
            email: String::from("jhon_smith@example.com"),
            password: PASSWORD.into(),
        };

        let request = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&request_body);

        let (response, _) = send_request::<ResponseError>(request).await;
        assert_eq!(response, ResponseError::UserNotFound);
    }
}
