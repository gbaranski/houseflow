use crate::State;
use axum::{extract, response};
use houseflow_types::{
    auth::register::{Request, Response},
    errors::{AuthError, ServerError},
    User,
};
use rand::random;
use tracing::Level;

#[tracing::instrument(
    name = "Register",
    err,
    skip(state, request),
    fields(email = %request.email, username = %request.username),
    err,
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

    tracing::event!(
        Level::INFO,
        user_id = %new_user.id,
        username = %new_user.username,
        email = %new_user.email,
        "Registered user"
    );

    Ok(response::Json(Response {
        user_id: new_user.id,
    }))
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test_utils::*;
    use axum::{extract, response};
    use houseflow_types::errors::{AuthError, ServerError};

    #[tokio::test]
    async fn valid() {
        let state = get_state();
        let request = Request {
            email: String::from("root@gbaranski.com"),
            username: String::from("Grzegorz Baranski"),
            password: PASSWORD.into(),
        };
        let response::Json(response) = super::handle(state.clone(), extract::Json(request.clone()))
            .await
            .unwrap();

        let db_user = state
            .database
            .get_user_by_email(&request.email)
            .unwrap()
            .expect("user not found in database");

        assert_eq!(db_user.id, response.user_id);
        assert_eq!(db_user.username, request.username);
        assert_eq!(db_user.email, request.email);
        assert!(crate::verify_password(&db_user.password_hash, PASSWORD).is_ok());
    }

    #[tokio::test]
    async fn already_exists() {
        let state = get_state();
        let user = get_user();
        state.database.add_user(&user).unwrap();
        let response = super::handle(
            state.clone(),
            extract::Json(Request {
                email: user.email,
                username: user.username,
                password: PASSWORD.into(),
            }),
        )
        .await
        .unwrap_err();
        assert_eq!(
            response,
            ServerError::AuthError(AuthError::UserAlreadyExists)
        );
    }
}
