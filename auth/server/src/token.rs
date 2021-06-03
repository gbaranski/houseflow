use crate::{AppData, TokenStore};
use actix_web::{
    post,
    web::{self, Form, FormConfig},
    App, HttpServer,
};
use houseflow_auth_types::{
    AccessTokenRequestBody, AccessTokenRequestError, AccessTokenRequestErrorKind,
    AccessTokenResponse, AccessTokenResponseBody, GrantType, TokenType,
};
use houseflow_token::{
    ExpirationDate, Payload as TokenPayload, Signature as TokenSignature, Token, TokenID,
};
use houseflow_types::{UserAgent, UserID};

pub fn exchange_refresh_token_form_config() -> FormConfig {
    FormConfig::default().error_handler(|err, _| {
        actix_web::Error::from(AccessTokenRequestError {
            error: AccessTokenRequestErrorKind::InvalidRequest,
            error_description: Some(err.to_string()),
        })
    })
}

#[derive(Debug, thiserror::Error)]
pub enum RefreshTokenExchangeError {
    #[error("invalid request: `{0}`")]
    InvalidRequest(#[from] AccessTokenRequestError),

    #[error("error with token_store: `{0}`")]
    TokenStoreError(#[from] crate::token_store::Error),
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
    request: Form<AccessTokenRequestBody>,
    token_store: web::Data<Box<dyn TokenStore>>,
    app_data: web::Data<AppData>,
) -> Result<web::Json<AccessTokenResponseBody>, RefreshTokenExchangeError> {
    use std::convert::TryFrom;
    use std::time::{Duration, SystemTime};

    let refresh_token = &request.refresh_token;
    refresh_token
        .verify(&app_data.refresh_key, None)
        .map_err(|err| AccessTokenRequestError {
            error: AccessTokenRequestErrorKind::InvalidGrant,
            error_description: Some(err.to_string()),
        })?;

    let stored_refresh_token = token_store
        .get(&refresh_token.payload.id)
        .await?
        .ok_or_else(|| AccessTokenRequestError {
            error: AccessTokenRequestErrorKind::InvalidGrant,
            error_description: Some("token does not exists in store".into()),
        })?;

    if *refresh_token != stored_refresh_token {
        return Err(AccessTokenRequestError {
            error: AccessTokenRequestErrorKind::InvalidGrant,
            error_description: Some("token does not match with this one in store".into()),
        }
        .into());
    }

    let expires_in = match refresh_token.payload.user_agent {
        UserAgent::Internal => Some(Duration::from_secs(3600)),
        UserAgent::GoogleSmartHome => None,
        UserAgent::None => todo!(),
    };

    let expires_at = ExpirationDate::from_duration(expires_in);
    let access_token_payload = TokenPayload {
        id: refresh_token.payload.user_id.clone(),
        user_agent: refresh_token.payload.user_agent,
        user_id: refresh_token.payload.user_id.clone(),
        expires_at,
    };
    let access_token_signature = access_token_payload.sign(&app_data.access_key);
    let access_token = Token::new(access_token_payload, access_token_signature);
    Ok(web::Json(AccessTokenResponseBody {
        access_token,
        token_type: TokenType::Bearer,
        expires_in,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryTokenStore;
    use actix_web::test;
    use rand::{random, Rng, RngCore};
    use std::time::{Duration, SystemTime};

    #[actix_rt::test]
    async fn test_exchange_refresh_token() {
        let token_store: web::Data<Box<dyn TokenStore>> =
            web::Data::new(Box::new(MemoryTokenStore::new()));
        let mut app_data = crate::AppData {
            refresh_key: vec![0; 32],
            access_key: vec![0; 32],
        };
        rand::thread_rng().fill_bytes(&mut app_data.refresh_key);
        rand::thread_rng().fill_bytes(&mut app_data.access_key);

        let refresh_token_payload = TokenPayload {
            id: TokenID::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            user_agent: UserAgent::Internal,
            user_id: UserID::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14, 15, 16]),
            expires_at: ExpirationDate::from_duration(Some(Duration::from_secs(10))),
        };
        println!("rtoken id: {:?}", refresh_token_payload.id);
        println!("ruser id: {:?}", refresh_token_payload.user_id);
        let refresh_token_signature = refresh_token_payload.sign(&app_data.refresh_key);
        let refresh_token = Token {
            payload: refresh_token_payload,
            signature: refresh_token_signature,
        };
        token_store.add(&refresh_token).await.unwrap();
        let mut app = test::init_service(
            App::new().configure(|cfg| crate::config(cfg, token_store.clone(), app_data.clone())),
        )
        .await;
        let request_body = AccessTokenRequestBody {
            grant_type: GrantType::RefreshToken,
            refresh_token,
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
        let verify_result = response_body.access_token.verify(
            &app_data.access_key,
            Some(&request_body.refresh_token.payload.user_agent),
        );
        assert!(
            verify_result.is_ok(),
            "failed access token verification: `{}`",
            verify_result.err().unwrap()
        );
        assert!(response_body.expires_in.is_some());
        assert!(response_body.expires_in.is_some());
        assert_eq!(
            response_body.access_token.payload.user_agent,
            request_body.refresh_token.payload.user_agent
        );
        assert_eq!(
            response_body.access_token.payload.user_id,
            request_body.refresh_token.payload.user_id
        );
    }
}
