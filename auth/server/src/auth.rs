use crate::{AppData, TokenStore};
use actix_files::NamedFile;
use actix_web::{
    get, post,
    web::{self, Data, Json, Query},
};
use houseflow_auth_types::{
    LoginError, LoginRequest, LoginResponseBody, RegisterError, RegisterRequest,
    RegisterResponseBody,
};
use houseflow_db::Database;
use houseflow_token::{ExpirationDate, Payload as TokenPayload, Token};
use houseflow_types::User;
use rand::random;

struct SomeWrapper([u8; 10]);

impl std::ops::Deref for SomeWrapper {
    type Target = [u8; 10];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn verify_password(hash: &str, password: &str) -> Result<(), LoginError> {
    match argon2::verify_encoded(hash, password.as_bytes()).unwrap() {
        true => Ok(()),
        false => Err(LoginError::InvalidPassword),
    }
}

#[post("/login")]
pub async fn login(
    request: Json<LoginRequest>,
    // _token_store: Data<dyn TokenStore>,
    app_data: Data<AppData>,
    db: Data<dyn Database>,
) -> Result<Json<LoginResponseBody>, LoginError> {
    let user = db
        .get_user_by_email(&request.email)
        .await?
        .ok_or(LoginError::UserNotFound)?;
    verify_password(&user.password_hash, &request.password)?;
    let refresh_token =
        Token::new_refresh_token(&app_data.refresh_key, &user.id, &request.user_agent);
    let access_token = Token::new_access_token(&app_data.access_key, &user.id, &request.user_agent);
    let response = LoginResponseBody {
        refresh_token,
        access_token,
    };
    Ok(Json(response))
}

#[post("/register")]
pub async fn register(
    request: Json<RegisterRequest>,
    app_data: Data<AppData>,
    token_store: Data<dyn TokenStore>,
    db: Data<dyn Database>,
) -> Result<Json<RegisterResponseBody>, RegisterError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use crate::MemoryTokenStore;
    use actix_web::{test, App};
    use houseflow_db::MemoryDatabase;
    use rand::{random, Rng, RngCore};
    use std::time::{Duration, SystemTime};

    #[actix_rt::test]
    async fn test_login() {
        let token_store = get_token_store();
        let database = get_database();
        let app_data = get_app_data();
        let password = String::from("SomePassword");
        let password_hash = argon2::hash_encoded(
            password.as_bytes(),
            &app_data.password_salt,
            &argon2::Config::default(),
        )
        .unwrap();
        let user = User {
            id: random(),
            first_name: String::from("John"),
            last_name: String::from("Smith"),
            email: String::from("john_smith@example.com"),
            password_hash,
        };
        database.add_user(&user).await.unwrap();
        let mut app = test::init_service(
            App::new().configure(|cfg| crate::config(cfg, token_store, database, app_data.clone())),
        )
        .await;

        let request_body = LoginRequest {
            email: user.email,
            password,
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
        let response: LoginResponseBody = test::read_body_json(response).await;
        let (at, rt) = (response.access_token, response.refresh_token);
        let verify_result = at.verify(&app_data.access_key, Some(&request_body.user_agent));
        assert!(
            verify_result.is_ok(),
            "failed access token verification: `{}`",
            verify_result.err().unwrap()
        );
        assert_eq!(at.user_agent(), rt.user_agent());
        assert_eq!(at.user_id(), rt.user_id());
    }
}
