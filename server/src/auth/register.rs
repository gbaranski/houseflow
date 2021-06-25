use actix_web::{
    post,
    web::{Data, Json},
};
use auth_types::{RegisterRequest, RegisterResponse, RegisterResponseBody, RegisterResponseError};
use config::server::Secrets;
use db::Database;
use rand::random;
use types::User;

#[post("/register")]
pub async fn on_register(
    request: Json<RegisterRequest>,
    secrets: Data<Secrets>,
    db: Data<dyn Database>,
) -> Result<Json<RegisterResponse>, RegisterResponseError> {
    validator::Validate::validate(&request.0)?;

    let password_hash = argon2::hash_encoded(
        request.password.as_bytes(),
        secrets.password_salt.as_bytes(),
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
        db::Error::AlreadyExists => RegisterResponseError::UserAlreadyExists,
        _ => err.into(),
    })?;

    let response = RegisterResponse::Ok(RegisterResponseBody {
        user_id: new_user.id,
    });
    Ok(Json(response))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test_utils::*;
//     use actix_web::{test, App};

//     #[actix_rt::test]
//     async fn test_register() {
//         let token_store = get_token_store();
//         let database = get_database();
//         let secrets: Secrets = random();
//         let mut app = test::init_service(App::new().configure(|cfg| {
//             crate::configure(cfg, token_store.clone(), database.clone(), secrets.clone())
//         }))
//         .await;

//         let request_body = RegisterRequest {
//             password: PASSWORD.into(),
//             username: String::from("John Smith"),
//             email: String::from("john_smith@example.com"),
//         };
//         let request = test::TestRequest::post()
//             .uri("/register")
//             .set_json(&request_body)
//             .to_request();
//         let response = test::call_service(&mut app, request).await;
//         assert_eq!(
//             response.status(),
//             200,
//             "status is not succesfull, body: {:?}",
//             test::read_body(response).await
//         );
//         let response: RegisterResponseBody = test::read_body_json(response).await;
//         let db_user = database
//             .get_user_by_email(&request_body.email)
//             .await
//             .unwrap()
//             .expect("user not found in database");

//         assert_eq!(db_user.id, response.user_id);
//         assert_eq!(db_user.username, request_body.username);
//         assert_eq!(db_user.email, request_body.email);
//         assert!(argon2::verify_encoded(&db_user.password_hash, PASSWORD.as_bytes()).unwrap());
//     }
// }
