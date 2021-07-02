use actix_web::{
    post,
    web::{Data, Json},
};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::auth::{
    RegisterRequest, RegisterResponse, RegisterResponseBody, RegisterResponseError,
};
use houseflow_types::User;
use rand::random;

#[post("/register")]
pub async fn on_register(
    request: Json<RegisterRequest>,
    config: Data<Config>,
    db: Data<dyn Database>,
) -> Result<Json<RegisterResponse>, RegisterResponseError> {
    validator::Validate::validate(&request.0)?;

    let password_hash = argon2::hash_encoded(
        request.password.as_bytes(),
        config.secrets.password_salt.as_bytes(),
        &argon2::Config::default(),
    )
    .unwrap();

    let new_user = User {
        id: random(),
        username: request.username.clone(),
        email: request.email.clone(),
        password_hash,
    };
    db.add_user(&new_user).await.map_err(|err| match err {
        houseflow_db::Error::AlreadyExists => RegisterResponseError::UserAlreadyExists,
        _ => RegisterResponseError::InternalError(err.to_string()),
    })?;

    let response = RegisterResponse::Ok(RegisterResponseBody {
        user_id: new_user.id,
    });
    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_register() {
        let request_body = RegisterRequest {
            password: PASSWORD.into(),
            username: String::from("John Smith"),
            email: String::from("john_smith@example.com"),
        };
        let request = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&request_body);
        let (response, state) = send_request::<RegisterResponseBody>(request).await;

        let db_user = state
            .database
            .get_user_by_email(&request_body.email)
            .await
            .unwrap()
            .expect("user not found in database");

        assert_eq!(db_user.id, response.user_id);
        assert_eq!(db_user.username, request_body.username);
        assert_eq!(db_user.email, request_body.email);
        assert!(argon2::verify_encoded(&db_user.password_hash, PASSWORD.as_bytes()).unwrap());
    }
}
