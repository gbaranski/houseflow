use crate::{token_store::Error as TokenStoreError, TokenStore};
use actix_web::web::{self, Data, Form, FormConfig, Json};
use chrono::{Duration, Utc};
use houseflow_config::server::Config;
use houseflow_types::{
    auth::token::{ResponseBody, ResponseError, TokenType},
    token::{
        AccessToken, AccessTokenPayload, AuthorizationCode, RefreshToken, RefreshTokenPayload,
    },
};
use serde::{Deserialize, Serialize};

use url::Url;

pub async fn on_refresh_token(
    Json(request): Form<Request>,
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
) -> Result<Json<ResponseBody>, ResponseError> {
    match request {
        Request::RefreshToken {
            refresh_token,
            client_id,
            client_secret,
            ..
        } => {
            if client_id != config.google.as_ref().unwrap().client_id
                || client_secret != config.google.as_ref().unwrap().client_secret
            {
                return Err(ResponseError::InvalidClient(None));
            }

            let refresh_token =
                RefreshToken::decode(config.secrets.refresh_key.as_bytes(), &refresh_token)
                    .map_err(|err| {
                        ResponseError::InvalidGrant(Some(format!(
                            "invalid refresh token: {}",
                            err.to_string()
                        )))
                    })?;

            // if !token_store
            //     .exists(&refresh_token.tid)
            //     .await
            //     .map_err(|err| err.into_internal_server_error())?
            // {
            //     return Err(ResponseError::InvalidGrant(Some(
            //         "refresh token is not present in store".into(),
            //     )));
            // }

            let expires_in = Duration::minutes(10);
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
                refresh_token: None,
            }))
        }
        Request::AuthorizationCode {
            client_id,
            client_secret,
            code,
            ..
        } => {
            if client_id != config.google.as_ref().unwrap().client_id
                || client_secret != config.google.as_ref().unwrap().client_secret
            {
                return Err(ResponseError::InvalidClient(None));
            }
            let code =
                AuthorizationCode::decode(config.secrets.authorization_code_key.as_bytes(), &code)
                    .map_err(|err| {
                        ResponseError::InvalidGrant(Some(format!(
                            "invalid authorization code: {}",
                            err.to_string()
                        )))
                    })?;

            let expires_in = Duration::minutes(10);
            let access_token = AccessToken::new(
                config.secrets.access_key.as_bytes(),
                AccessTokenPayload {
                    sub: code.sub.clone(),
                    exp: Utc::now() + expires_in,
                },
            );

            let refresh_token = RefreshToken::new(
                config.secrets.refresh_key.as_bytes(),
                RefreshTokenPayload {
                    sub: code.sub.clone(),
                    exp: None,
                    tid: rand::random(),
                },
            );
            token_store
                .add(&refresh_token.tid, refresh_token.exp.as_ref())
                .await
                .map_err(TokenStoreError::into_internal_server_error)?;

            Ok(web::Json(ResponseBody {
                access_token: access_token.to_string(),
                refresh_token: Some(refresh_token.to_string()),
                token_type: TokenType::Bearer,
                expires_in: Some(expires_in),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use houseflow_types::token::RefreshTokenPayload;

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
        state
            .token_store
            .add(&refresh_token.tid, refresh_token.exp.as_ref())
            .await
            .unwrap();
        let google_config = state.config.google.as_ref().unwrap();
        let response = on_token_grant(
            Form(Request::RefreshToken {
                refresh_token: refresh_token.to_string(),
                client_id: google_config.client_id.clone(),
                client_secret: google_config.client_secret.clone(),
                scope: None,
            }),
            state.token_store.clone(),
            state.config.clone(),
        )
        .await
        .unwrap()
        .into_inner();

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
        let google_config = state.config.google.as_ref().unwrap();
        let response = on_token_grant(
            Form(Request::RefreshToken {
                refresh_token: refresh_token.to_string(),
                client_id: google_config.client_id.clone(),
                client_secret: google_config.client_secret.clone(),
                scope: None,
            }),
            state.token_store.clone(),
            state.config.clone(),
        )
        .await
        .unwrap_err();

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
        let google_config = state.config.google.as_ref().unwrap();
        let response = on_token_grant(
            Form(Request::RefreshToken {
                refresh_token: refresh_token.to_string(),
                client_id: google_config.client_id.clone(),
                client_secret: google_config.client_secret.clone(),
                scope: None,
            }),
            state.token_store.clone(),
            state.config.clone(),
        )
        .await
        .unwrap_err();

        assert!(matches!(response, ResponseError::InvalidGrant(..)));
    }
}
