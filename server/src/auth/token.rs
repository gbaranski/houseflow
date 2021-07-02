use crate::{token_store, TokenStore};
use actix_web::{
    post,
    web::{self, Data, Form, FormConfig, Json},
};
use houseflow_config::server::Config;
use houseflow_types::{
    auth::{
        AccessTokenRequest, AccessTokenResponse, AccessTokenResponseBody, AccessTokenResponseError,
        TokenType,
    },
    token::{self, Token},
};

pub fn on_exchange_refresh_token_form_config() -> FormConfig {
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
    TokenStoreError(#[from] token_store::Error),
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
pub async fn on_exchange_refresh_token(
    request: Form<AccessTokenRequest>,
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
) -> Result<Json<AccessTokenResponse>, RefreshTokenExchangeError> {
    let refresh_token = &request.refresh_token;
    refresh_token
        .verify(&config.secrets.refresh_key, None)
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
    let expires_at = token::ExpirationDate::from_duration(expires_in);
    let access_token_payload = token::Payload {
        id: refresh_token.user_id().clone(),
        user_agent: *refresh_token.user_agent(),
        user_id: refresh_token.user_id().clone(),
        expires_at,
    };
    let access_token_signature = access_token_payload.sign(&config.secrets.access_key);
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
    use actix_web::test;
    use houseflow_types::auth::GrantType;
    use rand::random;
    use std::time::{Duration, SystemTime};

    #[actix_rt::test]
    async fn test_exchange_refresh_token() {
        let state = get_state();
        let refresh_token =
            Token::new_refresh_token(&state.config.secrets.refresh_key, &random(), &random());
        state.token_store.add(&refresh_token).await.unwrap();
        let request_body = AccessTokenRequest {
            grant_type: GrantType::RefreshToken,
            refresh_token: refresh_token.clone(),
        };
        let request = test::TestRequest::post()
            .uri("/auth/token")
            .set_form(&request_body);
        let response = send_request_with_state::<AccessTokenResponseBody>(request, &state).await;
        let access_token = &response.access_token;
        let verify_result = response.access_token.verify(
            &state.config.secrets.access_key,
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
        let state = get_state();
        let refresh_token =
            Token::new_refresh_token(&state.config.secrets.refresh_key, &random(), &random());
        let request_body = AccessTokenRequest {
            grant_type: GrantType::RefreshToken,
            refresh_token,
        };
        let request = test::TestRequest::post()
            .uri("/auth/token")
            .set_form(&request_body);
        let response = send_request_with_state::<AccessTokenResponseError>(request, &state).await;

        assert!(matches!(
            response,
            AccessTokenResponseError::InvalidGrant(..)
        ));
    }

    #[actix_rt::test]
    async fn test_exchange_refresh_token_expired_token() {
        let state = get_state();
        let refresh_token_payload = token::Payload::new(
            random(),
            random(),
            token::ExpirationDate::from_system_time(
                SystemTime::now().checked_sub(Duration::from_secs(5)),
            ),
        );
        let refresh_token_signature = refresh_token_payload.sign(&state.config.secrets.refresh_key);
        let refresh_token = Token::new(refresh_token_payload, refresh_token_signature);
        state.token_store.add(&refresh_token).await.unwrap();
        let request_body = AccessTokenRequest {
            grant_type: GrantType::RefreshToken,
            refresh_token: refresh_token.clone(),
        };
        let request = test::TestRequest::post()
            .uri("/auth/token")
            .set_form(&request_body);
        let response = send_request_with_state::<AccessTokenResponseError>(request, &state).await;
        assert!(matches!(
            response,
            AccessTokenResponseError::InvalidGrant(..)
        ));
    }
}
