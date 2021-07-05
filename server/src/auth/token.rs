use crate::TokenStore;
use actix_web::{
    post,
    web::{self, Data, Form, FormConfig, Json},
};
use chrono::{Duration, Utc};
use houseflow_config::server::Config;
use houseflow_types::{
    auth::token::{Request, ResponseBody, ResponseError, TokenType},
    token::{AccessToken, AccessTokenPayload, RefreshToken},
};

pub fn on_exchange_refresh_token_form_config() -> FormConfig {
    FormConfig::default().error_handler(|err, _| {
        actix_web::Error::from(ResponseError::InvalidRequest(Some(err.to_string())))
    })
}

#[post("/token")]
pub async fn on_exchange_refresh_token(
    Form(request): Form<Request>,
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
) -> Result<Json<ResponseBody>, ResponseError> {
    let expires_in = Duration::minutes(10);

    let refresh_token = RefreshToken::decode(
        config.secrets.refresh_key.as_bytes(),
        &request.refresh_token,
    )
    .map_err(|err| {
        ResponseError::InvalidGrant(Some(format!("invalid refresh token: {}", err.to_string())))
    })?;

    if !token_store
        .exists(&refresh_token.tid)
        .await
        .map_err(|err| err.into_internal_server_error())?
    {
        return Err(ResponseError::InvalidGrant(Some(
            "refresh token is not present in store".into(),
        )));
    }
    let access_token = AccessToken::new(
        config.secrets.access_key.as_bytes(),
        AccessTokenPayload {
            sub: refresh_token.sub.clone(),
            exp: Utc::now() + expires_in,
        },
    );

    Ok(web::Json(ResponseBody {
        access_token: access_token.to_string(),
        token_type: TokenType::Bearer,
        expires_in: Some(expires_in),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use actix_web::test;
    use houseflow_types::{auth::token::GrantType, token::RefreshTokenPayload};

    #[actix_rt::test]
    async fn test_exchange_refresh_token() {
        let state = get_state();
        let refresh_token = RefreshToken::new(
            state.config.secrets.refresh_key.as_bytes(),
            RefreshTokenPayload {
                tid: rand::random(),
                sub: rand::random(),
                exp: Some(Utc::now() + Duration::days(7)),
            },
        );
        state.token_store.add(&refresh_token.tid).await.unwrap();
        let request_body = Request {
            grant_type: GrantType::RefreshToken,
            refresh_token: refresh_token.to_string(),
        };
        let request = test::TestRequest::post()
            .uri("/auth/token")
            .set_form(&request_body);
        let response = send_request_with_state::<ResponseBody>(request, &state).await;
        let access_token = AccessToken::decode(
            state.config.secrets.access_key.as_bytes(),
            &response.access_token,
        )
        .unwrap();
        assert_eq!(access_token.sub, refresh_token.sub);
    }

    #[actix_rt::test]
    async fn test_exchange_refresh_token_not_existing_token() {
        let state = get_state();
        let refresh_token = RefreshToken::new(
            state.config.secrets.refresh_key.as_bytes(),
            RefreshTokenPayload {
                tid: rand::random(),
                sub: rand::random(),
                exp: Some(Utc::now() + Duration::days(7)),
            },
        );
        let request_body = Request {
            grant_type: GrantType::RefreshToken,
            refresh_token: refresh_token.to_string(),
        };
        let request = test::TestRequest::post()
            .uri("/auth/token")
            .set_form(&request_body);
        let response = send_request_with_state::<ResponseError>(request, &state).await;

        assert!(matches!(response, ResponseError::InvalidGrant(..)));
    }

    #[actix_rt::test]
    async fn test_exchange_refresh_token_expired_token() {
        let state = get_state();
        let refresh_token = RefreshToken::new(
            state.config.secrets.refresh_key.as_bytes(),
            RefreshTokenPayload {
                tid: rand::random(),
                sub: rand::random(),
                exp: Some(Utc::now() - Duration::hours(1)),
            },
        );
        state.token_store.add(&refresh_token.tid).await.unwrap();
        let request_body = Request {
            grant_type: GrantType::RefreshToken,
            refresh_token: refresh_token.to_string(),
        };
        let request = test::TestRequest::post()
            .uri("/auth/token")
            .set_form(&request_body);
        let response = send_request_with_state::<ResponseError>(request, &state).await;
        assert!(matches!(response, ResponseError::InvalidGrant(..)));
    }
}
