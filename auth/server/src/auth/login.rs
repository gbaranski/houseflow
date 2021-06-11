use crate::AppData;
use actix_web::{
    post,
    web::{Data, Json},
};
use auth_types::{
    LoginRequest, LoginResponse, LoginResponseBody, LoginResponseError,
};
use db::Database;

use token::store::TokenStore;
use token::Token;

fn verify_password(hash: &str, password: &str) -> Result<(), auth_types::LoginResponseError> {
    match argon2::verify_encoded(hash, password.as_bytes()).unwrap() {
        true => Ok(()),
        false => Err(auth_types::LoginResponseError::InvalidPassword),
    }
}


#[post("/login")]
pub async fn login(
    request: Json<LoginRequest>,
    token_store: Data<dyn TokenStore>,
    app_data: Data<AppData>,
    db: Data<dyn Database>,
) -> Result<Json<LoginResponse>, LoginResponseError> {
    let user = db
        .get_user_by_email(&request.email)
        .await?
        .ok_or(LoginResponseError::UserNotFound)?;
    verify_password(&user.password_hash, &request.password)?;
    let refresh_token =
        Token::new_refresh_token(&app_data.refresh_key, &user.id, &request.user_agent);
    let access_token = Token::new_access_token(&app_data.access_key, &user.id, &request.user_agent);
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
    use actix_web::{test, App};
    use types::User;

    use rand::random;

    #[actix_rt::test]
    async fn test_login() {
        let token_store = get_token_store();
        let database = get_database();
        let app_data = get_app_data();
        let user = User {
            id: random(),
            username: String::from("John Smith"),
            email: String::from("john_smith@example.com"),
            password_hash: PASSWORD_HASH.into(),
        };
        database.add_user(&user).await.unwrap();
        let mut app =
            test::init_service(App::new().configure(|cfg| {
                crate::config(cfg, token_store.clone(), database, app_data.clone())
            }))
            .await;

        let request_body = LoginRequest {
            email: user.email,
            password: PASSWORD.into(),
            user_agent: random(),
        };
        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(&request_body)
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(
            response.status(),
            200,
            "status is not succesfull, body: {:?}",
            test::read_body(response).await
        );
        let response: LoginResponse = test::read_body_json(response).await;
        let response = match response {
            LoginResponse::Ok(response) => response,
            LoginResponse::Err(err) => panic!("unexpected login error: {:?}", err),
        };
        let (at, rt) = (response.access_token, response.refresh_token);
        let verify_result = at.verify(&app_data.access_key, Some(&request_body.user_agent));
        assert!(
            verify_result.is_ok(),
            "failed access token verification: `{}`",
            verify_result.err().unwrap()
        );
        assert_eq!(at.user_agent(), rt.user_agent());
        assert_eq!(at.user_id(), rt.user_id());
        assert!(
            !token_store.exists(at.id()).await.unwrap(),
            "access token found in token store"
        );
        assert!(
            token_store.exists(rt.id()).await.unwrap(),
            "refresh token not found in token store"
        );
    }

    #[actix_rt::test]
    async fn test_login_invalid_password() {
        let token_store = get_token_store();
        let database = get_database();
        let app_data = get_app_data();
        let user = User {
            id: random(),
            username: String::from("John Smith"),
            email: String::from("john_smith@example.com"),
            password_hash: PASSWORD_HASH.into(),
        };
        database.add_user(&user).await.unwrap();
        let mut app =
            test::init_service(App::new().configure(|cfg| {
                crate::config(cfg, token_store.clone(), database, app_data.clone())
            }))
            .await;

        let request_body = LoginRequest {
            email: user.email,
            password: PASSWORD_INVALID.into(),
            user_agent: random(),
        };
        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(&request_body)
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(
            response.status(),
            400,
            "unexpected status code, body: {:?}",
            test::read_body(response).await
        );
        let response: LoginResponse = test::read_body_json(response).await;
        match response {
            LoginResponse::Err(LoginResponseError::InvalidPassword) => (),
            _ => panic!("unexpected response returned: {:?}", response),
        }
    }

    #[actix_rt::test]
    async fn test_login_not_existing_user() {
        let token_store = get_token_store();
        let database = get_database();
        let app_data = get_app_data();
        let user = User {
            id: random(),
            username: String::from("John Smith"),
            email: String::from("john_smith@example.com"),
            password_hash: PASSWORD_HASH.into(),
        };
        let mut app =
            test::init_service(App::new().configure(|cfg| {
                crate::config(cfg, token_store.clone(), database, app_data.clone())
            }))
            .await;

        let request_body = LoginRequest {
            email: user.email,
            password: PASSWORD.into(),
            user_agent: random(),
        };
        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(&request_body)
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(
            response.status(),
            404,
            "unexpected status code, body: {:?}",
            test::read_body(response).await
        );
        let response: LoginResponse = test::read_body_json(response).await;
        match response {
            LoginResponse::Err(LoginResponseError::UserNotFound) => (),
            _ => panic!("invalid response returned: {:?}", response),
        }
    }
}
