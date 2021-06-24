use crate::TokenStore;
use actix_web::{
    post,
    web::{self, Data, Form, FormConfig, Json},
};
use auth_types::{
    AccessTokenRequest, AccessTokenResponse, AccessTokenResponseBody, AccessTokenResponseError,
    TokenType,
};
use config::server::Secrets;
use token::{ExpirationDate, Payload as TokenPayload, Token};

pub fn exchange_refresh_token_form_config() -> FormConfig {
    FormConfig::default().error_handler(|err, _| {
        actix_web::Error::from(AccessTokenResponseError::InvalidRequest(Some(
            err.to_string(),
        )))
    })
}

#[derive(Debug, thiserror::Error)]
pub enum RefreshTokenExchangeError {
    #[error("invalid request: `{0}`")]
    InvalidRequest(#[from] AccessTokenResponseError),

    #[error("error with token_store: `{0}`")]
    TokenStoreError(#[from] token::store::Error),
}

impl actix_web::ResponseError for RefreshTokenExchangeError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;
        match self {
            Self::InvalidRequest(err) => err.status_code(),
            Self::TokenStoreError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            Self::InvalidRequest(err) => err.error_response(),
            Self::TokenStoreError(err) => {
                actix_web::HttpResponse::build(self.status_code()).body(err.to_string())
            }
        }
    }
}

#[post("/token")]
pub async fn exchange_refresh_token(
    request: Form<AccessTokenRequest>,
    token_store: Data<dyn TokenStore>,
    secrets: Data<Secrets>,
) -> Result<Json<AccessTokenResponse>, RefreshTokenExchangeError> {
    let refresh_token = &request.refresh_token;
    refresh_token
        .verify(&secrets.refresh_key, None)
        .map_err(|err| AccessTokenResponseError::InvalidGrant(Some(err.to_string())))?;

    let stored_refresh_token = token_store.get(&refresh_token.id()).await?.ok_or_else(|| {
        AccessTokenResponseError::InvalidGrant(Some("token does not exists in store".into()))
    })?;

    if *refresh_token != stored_refresh_token {
        return Err(AccessTokenResponseError::InvalidGrant(Some(
            "token does not match with this one in store".into(),
        ))
        .into());
    }

    let expires_in = refresh_token.user_agent().access_token_duration();
    let expires_at = ExpirationDate::from_duration(expires_in);
    let access_token_payload = TokenPayload {
        id: refresh_token.user_id().clone(),
        user_agent: *refresh_token.user_agent(),
        user_id: refresh_token.user_id().clone(),
        expires_at,
    };
    let access_token_signature = access_token_payload.sign(&secrets.access_key);
    let access_token = Token::new(access_token_payload, access_token_signature);
    Ok(web::Json(AccessTokenResponse::Ok(
        AccessTokenResponseBody {
            access_token,
            token_type: TokenType::Bearer,
            expires_in,
        },
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use actix_web::{test, App};
    use auth_types::GrantType;
    use rand::random;
    use std::time::{Duration, SystemTime};

    #[actix_rt::test]
    async fn test_exchange_refresh_token() {
        let token_store = get_token_store();
        let database = get_database();
        let secrets: Secrets = random();
        let refresh_token = Token::new_refresh_token(&secrets.refresh_key, &random(), &random());
        token_store.add(&refresh_token).await.unwrap();
        let mut app = test::init_service(
            App::new()
                .configure(|cfg| crate::configure(cfg, token_store, database, secrets.clone())),
        )
        .await;
        let request_body = AccessTokenRequest {
            grant_type: GrantType::RefreshToken,
            refresh_token: refresh_token.clone(),
        };
        let request = test::TestRequest::post()
            .uri("/token")
            .set_form(&request_body)
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(
            response.status(),
            200,
            "status is not succesfull, body: {:?}",
            test::read_body(response).await
        );
        let response_body: AccessTokenResponseBody = test::read_body_json(response).await;
        let access_token = &response_body.access_token;
        let verify_result = response_body.access_token.verify(
            &secrets.access_key,
            Some(&request_body.refresh_token.user_agent()),
        );
        assert!(
            verify_result.is_ok(),
            "failed access token verification: `{}`",
            verify_result.err().unwrap()
        );
        assert_eq!(access_token.user_agent(), refresh_token.user_agent());
        assert_eq!(access_token.user_id(), refresh_token.user_id());
    }

    #[actix_rt::test]
    async fn test_exchange_refresh_token_not_existing_token() {
        let token_store = get_token_store();
        let database = get_database();
        let secrets: Secrets = random();
        let refresh_token = Token::new_refresh_token(&secrets.refresh_key, &random(), &random());
        let mut app = test::init_service(App::new().configure(|cfg| {
            crate::configure(cfg, token_store.clone(), database.clone(), secrets.clone())
        }))
        .await;
        let request_body = AccessTokenRequest {
            grant_type: GrantType::RefreshToken,
            refresh_token,
        };
        let request = test::TestRequest::post()
            .uri("/token")
            .set_form(&request_body)
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), 400);
        let response_body: AccessTokenResponseError = test::read_body_json(response).await;
        match response_body {
            AccessTokenResponseError::InvalidGrant(_) => (),
            _ => panic!("unexpected error received: {:?}", response_body),
        }
    }

    #[actix_rt::test]
    async fn test_exchange_refresh_token_expired_token() {
        let token_store = get_token_store();
        let database = get_database();
        let secrets: Secrets = random();
        let refresh_token_payload = TokenPayload::new(
            random(),
            random(),
            ExpirationDate::from_system_time(SystemTime::now().checked_sub(Duration::from_secs(5))),
        );
        let refresh_token_signature = refresh_token_payload.sign(&secrets.refresh_key);
        let refresh_token = Token::new(refresh_token_payload, refresh_token_signature);
        token_store.add(&refresh_token).await.unwrap();
        let mut app = test::init_service(App::new().configure(|cfg| {
            crate::configure(cfg, token_store.clone(), database.clone(), secrets.clone())
        }))
        .await;
        let request_body = AccessTokenRequest {
            grant_type: GrantType::RefreshToken,
            refresh_token,
        };
        let request = test::TestRequest::post()
            .uri("/token")
            .set_form(&request_body)
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), 400);
        let response_body: AccessTokenResponseError = test::read_body_json(response).await;
        match response_body {
            AccessTokenResponseError::InvalidGrant(_) => (),
            _ => panic!("unexpected error received: {:?}", response_body),
        }
    }
}
