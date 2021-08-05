use axum::{extract, response};
use crate::State;
use houseflow_types::{
    auth::register::{Request, Response},
    errors::{ServerError, AuthError}, User,
};
use rand::random;
use tracing::Level;

#[tracing::instrument(
    name = "Register",
    skip(state, request),
    fields(email = %request.email, username = %request.username)
)]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    extract::Json(request): extract::Json<Request>,
) -> Result<response::Json<Response>, ServerError> {
    validator::Validate::validate(&request)?;

    let password_hash = argon2::hash_encoded(
        request.password.as_bytes(),
        &crate::get_password_salt(),
        &argon2::Config::default(),
    )
    .unwrap();

    let new_user = User {
        id: random(),
        username: request.username.clone(),
        email: request.email,
        password_hash,
    };
    state
        .database
        .add_user(&new_user)
        .map_err(|err| match err {
            houseflow_db::Error::AlreadyExists => AuthError::UserAlreadyExists.into(),
            other => ServerError::from(other),
        })?;

    tracing::event!(Level::INFO, user_id = %new_user.id);

    Ok(response::Json(Response {
        user_id: new_user.id,
    }))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test_utils::*;
//
//     #[actix_rt::test]
//     async fn test_register() {
//         let state = get_state();
//         let request = Request {
//             email: String::from("john_smith@example.com"),
//             username: String::from("John Smith"),
//             password: PASSWORD.into(),
//         };
//         let response = on_register(Json(request.clone()), state.database.clone())
//             .await
//             .unwrap()
//             .into_inner();
//
//         let db_user = state
//             .database
//             .get_user_by_email(&request.email)
//             .unwrap()
//             .expect("user not found in database");
//
//         assert_eq!(db_user.id, response.user_id);
//         assert_eq!(db_user.username, request.username);
//         assert_eq!(db_user.email, request.email);
//         assert!(argon2::verify_encoded(&db_user.password_hash, PASSWORD.as_bytes()).unwrap());
//     }
// }
