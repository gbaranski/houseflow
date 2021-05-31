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

#[post("/token")]
pub async fn exchange_refresh_token(
    request: Form<AccessTokenRequestBody>,
    token_store: web::Data<Box<dyn TokenStore>>,
    app_data: web::Data<AppData>,
) -> Result<web::Json<AccessTokenResponseBody>, AccessTokenRequestError> {
    use std::convert::TryFrom;
    use std::time::{Duration, SystemTime};

    let expires_in = Duration::from_secs(3600);
    let payload = TokenPayload {
        id: rand::random(),
        user_agent: request.refresh_token.payload.user_agent,
        user_id: request.refresh_token.payload.user_id.clone(),
        expires_at: ExpirationDate::from(
            SystemTime::now().checked_add(expires_in.clone()).unwrap(),
        ),
    };
    let signature = payload.sign(&app_data.access_key);
    let token = Token::new(payload, signature);
    Ok(web::Json(AccessTokenResponseBody {
        access_token: token,
        token_type: TokenType::Bearer,
        expires_in: Some(expires_in),
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
            id: random(),
            user_agent: UserAgent::Internal,
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_add(Duration::from_secs(10))
                .unwrap()
                .into(),
        };
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
            &request_body.refresh_token.payload.user_agent,
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
