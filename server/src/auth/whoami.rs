use actix_web::{
    get,
    web::{Data, HttpRequest, Json},
};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    auth::{WhoamiResponse, WhoamiResponseBody, WhoamiResponseError},
    token::Token,
    UserAgent,
};

#[get("/whoami")]
pub async fn on_whoami(
    config: Data<Config>,
    db: Data<dyn Database>,
    req: HttpRequest,
) -> Result<Json<WhoamiResponse>, WhoamiResponseError> {
    let token = Token::from_request(&req)?;
    token.verify(&config.secrets.access_key, Some(&UserAgent::Internal))?;
    let user = db
        .get_user(token.user_id())
        .await
        .map_err(|err| WhoamiResponseError::InternalError(err.to_string()))?
        .ok_or(WhoamiResponseError::UserNotFound)?;

    let response = WhoamiResponseBody {
        username: user.username,
        email: user.email,
    };

    Ok(Json(WhoamiResponse::Ok(response)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use actix_web::{http, test};
    use houseflow_types::User;

    use rand::random;

    #[actix_rt::test]
    async fn valid() {
        let state = get_state();
        let user = User {
            id: random(),
            username: String::from("John Smith"),
            email: String::from("john_smith@example.com"),
            password_hash: PASSWORD_HASH.into(),
        };
        let token = Token::new_access_token(
            &state.config.secrets.access_key,
            &user.id,
            &UserAgent::Internal,
        );
        state.database.add_user(&user).await.unwrap();

        let request = test::TestRequest::get().uri("/auth/whoami").append_header((
            http::header::AUTHORIZATION,
            format!("Bearer {}", token.to_string()),
        ));

        let response = send_request_with_state::<WhoamiResponseBody>(request, &state).await;
        assert_eq!(user.email, response.email);
        assert_eq!(user.username, response.username);
    }

    #[actix_rt::test]
    async fn missing_header() {
        let request = test::TestRequest::get().uri("/auth/whoami");
        let (response, _) = send_request::<WhoamiResponseError>(request).await;
        assert!(matches!(
            response,
            WhoamiResponseError::DecodeHeaderError(_)
        ));
    }

    #[actix_rt::test]
    async fn invalid_token_signature() {
        let state = get_state();
        let user = User {
            id: random(),
            username: String::from("John Smith"),
            email: String::from("john_smith@example.com"),
            password_hash: PASSWORD_HASH.into(),
        };
        let token = Token::new_access_token(b"other_key", &user.id, &UserAgent::Internal);
        state.database.add_user(&user).await.unwrap();

        let request = test::TestRequest::get().uri("/auth/whoami").append_header((
            http::header::AUTHORIZATION,
            format!("Bearer {}", token.to_string()),
        ));

        let response = send_request_with_state::<WhoamiResponseError>(request, &state).await;
        assert!(matches!(response, WhoamiResponseError::VerifyError(_)));
    }
}
