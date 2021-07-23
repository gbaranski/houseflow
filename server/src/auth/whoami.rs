use actix_web::web::{Data, HttpRequest, Json};
use tracing::Level;
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    auth::whoami::{ResponseBody, ResponseError},
    token::AccessToken,
};

#[tracing::instrument(name = "Whoami", skip(config, db, http_request))]
pub async fn on_whoami(
    config: Data<Config>,
    db: Data<dyn Database>,
    http_request: HttpRequest,
) -> Result<Json<ResponseBody>, ResponseError> {
    let access_token =
        AccessToken::from_request(config.secrets.access_key.as_bytes(), &http_request)?;
    let user = db
        .get_user(&access_token.sub)
        .map_err(houseflow_db::Error::into_internal_server_error)?
        .ok_or(ResponseError::UserNotFound)?;

    tracing::event!(Level::INFO, user_id = %user.id);

    Ok(Json(ResponseBody {
        username: user.username,
        email: user.email,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use actix_web::{http, test};
    use chrono::{Duration, Utc};
    use houseflow_types::{token::AccessTokenPayload, User};

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

        let access_token = AccessToken::new(
            state.config.secrets.access_key.as_bytes(),
            AccessTokenPayload {
                sub: user.id.clone(),
                exp: Utc::now() + Duration::seconds(5),
            },
        );

        state.database.add_user(&user).unwrap();

        let request = test::TestRequest::default()
            .append_header((
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token.to_string()),
            ))
            .to_http_request();
        let response = on_whoami(state.config, state.database.clone(), request)
            .await
            .unwrap()
            .into_inner();

        assert_eq!(user.email, response.email);
        assert_eq!(user.username, response.username);
    }

    #[actix_rt::test]
    async fn missing_token() {
        let state = get_state();
        let request = test::TestRequest::default().to_http_request();
        let response = on_whoami(state.config, state.database, request)
            .await
            .unwrap_err();
        assert!(matches!(response, ResponseError::TokenError(_)));
    }

    #[actix_rt::test]
    async fn invalid_token_signature() {
        let state = get_state();
        let user = get_user();
        let access_token = AccessToken::new(
            &Vec::from("other key"),
            AccessTokenPayload {
                sub: user.id.clone(),
                exp: Utc::now() + Duration::seconds(5),
            },
        );

        state.database.add_user(&user).unwrap();
        let request = test::TestRequest::default()
            .append_header((
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token.to_string()),
            ))
            .to_http_request();
        let response = on_whoami(state.config, state.database, request)
            .await
            .unwrap_err();

        assert!(matches!(response, ResponseError::TokenError(_)));
    }
}
