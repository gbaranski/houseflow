use crate::TokenStore;
use actix_web::{
    post,
    web::{self, Form, FormConfig},
    App, HttpServer,
};
use houseflow_auth_types::{
    AccessTokenRequestBody, AccessTokenRequestError, AccessTokenRequestErrorKind,
    AccessTokenResponseBody, GrantType, TokenType,
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
) -> Result<web::Json<AccessTokenResponseBody>, AccessTokenRequestError> {
    println!("request: {:?}", request);
    use std::convert::TryFrom;
    use std::time::{Duration, SystemTime};
    let expires_in = Duration::from_secs(3600);
    let payload = TokenPayload {
        id: TokenID::try_from("1b83055496544bc4873b40054529417f").unwrap(),
        user_agent: UserAgent::GoogleSmartHome,
        user_id: UserID::try_from("476f5fbe25824291a5a87d8097071321").unwrap(),
        expires_at: ExpirationDate::from(
            SystemTime::now().checked_add(expires_in.clone()).unwrap(),
        ),
    };
    let signature = payload.sign(b"some-key");
    let token = Token::new(payload, signature);
    Ok(web::Json(AccessTokenResponseBody {
        access_token: token,
        token_type: TokenType::Bearer,
        expires_in: Some(expires_in),
    }))
    // Err(AccessTokenRequestError {
    //     error: AccessTokenRequestErrorKind::InvalidClient,
    //     error_description: Some("test".into()),
    // })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryTokenStore;
    use actix_web::test;
    use rand::random;
    use std::time::{Duration, SystemTime};

    const KEY: &[u8] = b"some key";

    #[actix_rt::test]
    async fn test_exchange_refresh_token() {
        let token_store: web::Data<Box<dyn TokenStore>> =
            web::Data::new(Box::new(MemoryTokenStore::new()));
        let refresh_token_payload = TokenPayload {
            id: random(),
            user_agent: UserAgent::Internal,
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_add(Duration::from_secs(10))
                .unwrap()
                .into(),
        };
        let refresh_token_signature = refresh_token_payload.sign(KEY);
        let refresh_token = Token {
            payload: refresh_token_payload,
            signature: refresh_token_signature,
        };
        token_store.add(&refresh_token).await.unwrap();
        let mut app =
            test::init_service(App::new().configure(|cfg| crate::config(cfg, token_store.clone())))
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
        let response_body: AccessTokenRequestError = test::read_body_json(response).await;
        assert_eq!(
            response_body.error,
            AccessTokenRequestErrorKind::InvalidRequest
        );
        assert!(response_body.error_description.is_some());
    }

    // #[actix_rt::test]
    // async fn test_exchange_refresh_token_empty_body() {
    //     let token_store = MemoryTokenStore::new();
    //     let mut app = test::init_service(
    //         App::new().service(
    //             web::scope("/")
    //                 .app_data(exchange_refresh_token_query_config())
    //                 .app_data(token_store)
    //                 .service(exchange_refresh_token),
    //         ),
    //     )
    //     .await;
    //     let request = test::TestRequest::post().uri("/token").to_request();
    //     let response = test::call_service(&mut app, request).await;
    //     assert_eq!(response.status(), 400);
    //     let response_body: AccessTokenRequestError = test::read_body_json(response).await;
    //     assert_eq!(
    //         response_body.error,
    //         AccessTokenRequestErrorKind::InvalidRequest
    //     );
    //     assert!(response_body.error_description.is_some());
    // }
}
