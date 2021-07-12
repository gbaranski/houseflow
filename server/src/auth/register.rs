use actix_web::web::{Data, Json};
use houseflow_db::Database;
use houseflow_types::auth::register::{Request, ResponseBody, ResponseError};
use houseflow_types::User;
use rand::random;

pub async fn on_register(
    Json(request): Json<Request>,
    db: Data<dyn Database>,
) -> Result<Json<ResponseBody>, ResponseError> {
    validator::Validate::validate(&request).map_err(houseflow_types::ValidationError::from)?;

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
    db.add_user(&new_user).map_err(|err| match err {
        houseflow_db::Error::AlreadyExists => ResponseError::UserAlreadyExists,
        other => other.into_internal_server_error().into(),
    })?;

    Ok(Json(ResponseBody {
        user_id: new_user.id,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[actix_rt::test]
    async fn test_register() {
        let state = get_state();
        let request = Request {
            email: String::from("john_smith@example.com"),
            username: String::from("John Smith"),
            password: PASSWORD.into(),
        };
        let response = on_register(Json(request.clone()), state.database.clone())
            .await
            .unwrap()
            .into_inner();

        let db_user = state
            .database
            .get_user_by_email(&request.email)
            .unwrap()
            .expect("user not found in database");

        assert_eq!(db_user.id, response.user_id);
        assert_eq!(db_user.username, request.username);
        assert_eq!(db_user.email, request.email);
        assert!(argon2::verify_encoded(&db_user.password_hash, PASSWORD.as_bytes()).unwrap());
    }
}
