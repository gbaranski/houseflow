use crate::TokenStore;
use actix_web::{
    post,
    web::{Data, Json},
};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    auth::{LoginRequest, LoginResponse, LoginResponseBody, LoginResponseError},
    token::Token,
};

fn verify_password(hash: &str, password: &str) -> Result<(), LoginResponseError> {
    match argon2::verify_encoded(hash, password.as_bytes()).unwrap() {
        true => Ok(()),
        false => Err(LoginResponseError::InvalidPassword),
    }
}

#[post("/login")]
pub async fn on_login(
    request: Json<LoginRequest>,
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
    db: Data<dyn Database>,
) -> Result<Json<LoginResponse>, LoginResponseError> {
    validator::Validate::validate(&request.0)?;
    let user = db
        .get_user_by_email(&request.email)
        .await
        .map_err(|err| LoginResponseError::InternalError(err.to_string()))?
        .ok_or(LoginResponseError::UserNotFound)?;

    verify_password(&user.password_hash, &request.password)?;
    let refresh_token =
        Token::new_refresh_token(&config.secrets.refresh_key, &user.id, &request.user_agent);
    let access_token =
        Token::new_access_token(&config.secrets.access_key, &user.id, &request.user_agent);
    token_store
        .add(&refresh_token)
        .await
        .map_err(|err| LoginResponseError::InternalError(err.to_string()))?;

    let response = LoginResponse::Ok(LoginResponseBody {
        refresh_token,
        access_token,
    });
    Ok(Json(response))
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

        let request_body = LoginRequest {
            email: user.email,
            password: PASSWORD.into(),
            user_agent: random(),
        };
        let request = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&request_body);
        let response = send_request_with_state::<LoginResponseBody>(request, &state).await;
        let (at, rt) = (response.access_token, response.refresh_token);
        let verify_result = at.verify(
            &state.config.secrets.access_key,
            Some(&request_body.user_agent),
        );
        assert!(
            verify_result.is_ok(),
            "failed access token verification: `{}`",
            verify_result.err().unwrap()
        );
        assert_eq!(at.user_agent(), rt.user_agent());
        assert_eq!(at.user_id(), rt.user_id());
        assert!(
            !state.token_store.exists(at.id()).await.unwrap(),
            "access token found in token store"
        );
        assert!(
            state.token_store.exists(rt.id()).await.unwrap(),
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

        let request_body = LoginRequest {
            email: user.email,
            password: PASSWORD_INVALID.into(),
            user_agent: random(),
        };
        let request = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&request_body);
        let response = send_request_with_state::<LoginResponseError>(request, &state).await;
        assert_eq!(response, LoginResponseError::InvalidPassword);
    }

    #[actix_rt::test]
    async fn test_login_not_existing_user() {
        let request_body = LoginRequest {
            email: String::from("jhon_smith@example.com"),
            password: PASSWORD.into(),
            user_agent: random(),
        };

        let request = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&request_body);

        let (response, _) = send_request::<LoginResponseError>(request).await;
        assert_eq!(response, LoginResponseError::UserNotFound);
    }
}
