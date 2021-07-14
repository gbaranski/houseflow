use crate::{token_store::Error as TokenStoreError, TokenStore};
use actix_web::web::{Data, Form, FormConfig, Json};
use chrono::{Duration, Utc};
use houseflow_config::server::Config;
use houseflow_types::token::{
    AccessToken, AccessTokenPayload, AuthorizationCode, RefreshToken, RefreshTokenPayload,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "grant_type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Request {
    RefreshToken {
        /// The client ID
        client_id: String,

        /// The client secret
        client_secret: String,

        /// The refresh token previously issued to the client.
        refresh_token: String,
    },

    AuthorizationCode {
        /// The client ID
        client_id: String,

        /// The client secret
        client_secret: String,

        /// This parameter is the authorization code that the client previously received from the authorization server.
        code: String,
    },
}

pub type Response = Result<ResponseBody, ResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseBody {
    /// The access token string as issued by the authorization server.
    pub access_token: String,

    /// The refresh token string as issued by the authorization server.
    pub refresh_token: Option<String>,

    /// The type of token this is, typically just the string “Bearer”.
    pub token_type: TokenType,

    /// If the access token expires, the server should reply with the duration of time the access token is granted for.
    #[serde(with = "houseflow_types::serde_token_expiration")]
    pub expires_in: Option<Duration>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TokenType {
    Bearer,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum ResponseError {
    #[error("internal error: {0}")]
    InternalError(#[from] houseflow_types::InternalServerError),

    /// The request is missing a parameter so the server can’t proceed with the request.
    /// This may also be returned if the request includes an unsupported parameter or repeats a parameter.
    #[error("invalid request, description: {0:?}")]
    InvalidRequest(Option<String>),

    /// Client authentication failed, such as if the request contains an invalid client ID or secret.
    /// Send an HTTP 401 response in this case.
    #[error("invalid clientid or secret, description: {0:?}")]
    InvalidClient(Option<String>),

    /// The authorization code (or user’s password for the password grant type) is invalid or expired.
    /// This is also the error you would return if the redirect URL given in the authorization grant does not match the URL provided in this access token request.
    #[error("invalid grant, description: {0:?}")]
    InvalidGrant(Option<String>),

    /// For access token requests that include a scope (password or client_credentials grants), this error indicates an invalid scope value in the request.
    #[error("invalid scope, description: {0:?}")]
    InvalidScope(Option<String>),

    /// This client is not authorized to use the requested grant type.
    /// For example, if you restrict which applications can use the Implicit grant, you would return this error for the other apps.
    #[error("unauthorized client, description: {0:?}")]
    UnauthorizedClient(Option<String>),

    /// If a grant type is requested that the authorization server doesn’t recognize, use this code.
    /// Note that unknown grant types also use this specific error code rather than using the invalid_request above.
    #[error("unsupported grant type, description: {0:?}")]
    UnsupportedGrantType(Option<String>),
}

impl actix_web::ResponseError for ResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let json = actix_web::web::Json(self);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}

pub fn on_token_grant_form_config() -> FormConfig {
    FormConfig::default().error_handler(|err, _| {
        actix_web::Error::from(ResponseError::InvalidRequest(Some(err.to_string())))
    })
}

async fn on_refresh_token_grant(
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
    refresh_token: String,
) -> Response {
    let refresh_token = RefreshToken::decode(config.secrets.refresh_key.as_bytes(), &refresh_token)
        .map_err(|err| {
            ResponseError::InvalidGrant(Some(format!("invalid refresh token: {}", err.to_string())))
        })?;

    if !token_store
        .exists(&refresh_token.tid)
        .await
        .map_err(TokenStoreError::into_internal_server_error)?
    {
        return Err(ResponseError::InvalidGrant(Some(
            "refresh token is not present in store".into(),
        )));
    }

    let expires_in = Duration::minutes(10);
    let access_token = AccessToken::new(
        config.secrets.access_key.as_bytes(),
        AccessTokenPayload {
            sub: refresh_token.sub.clone(),
            exp: Utc::now() + expires_in,
        },
    );

    Ok(ResponseBody {
        access_token: access_token.to_string(),
        token_type: TokenType::Bearer,
        expires_in: Some(expires_in),
        refresh_token: None,
    })
}

async fn on_authorization_code_grant(
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
    code: String,
) -> Response {
    let code = AuthorizationCode::decode(config.secrets.authorization_code_key.as_bytes(), &code)
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

    Ok(ResponseBody {
        access_token: access_token.to_string(),
        refresh_token: Some(refresh_token.to_string()),
        token_type: TokenType::Bearer,
        expires_in: Some(expires_in),
    })
}

pub async fn on_token_grant(
    Form(request): Form<Request>,
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
) -> Result<Json<ResponseBody>, ResponseError> {
    let verify_client = |client_id, client_secret| {
        if client_id != config.google.as_ref().unwrap().client_id
            || client_secret != config.google.as_ref().unwrap().client_secret
        {
            Err(ResponseError::InvalidClient(None))
        } else {
            Ok(())
        }
    };

    match request {
        Request::RefreshToken {
            refresh_token,
            client_id,
            client_secret,
            ..
        } => {
            verify_client(client_id, client_secret)?;
            on_refresh_token_grant(token_store, config, refresh_token).await
        }
        Request::AuthorizationCode {
            client_id,
            client_secret,
            code,
            ..
        } => {
            verify_client(client_id, client_secret)?;
            on_authorization_code_grant(token_store, config, code).await
        }
    }
    .map(Json)
}

#[cfg(test)]
mod tests {
    use houseflow_types::token::AuthorizationCodePayload;

    use super::*;
    use crate::test_utils::*;

    #[actix_rt::test]
    async fn valid() {
        let state = get_state();
        let google_config = state.config.google.as_ref().unwrap();

        let code_payload = AuthorizationCodePayload {
            sub: rand::random(),
            exp: Utc::now() + Duration::minutes(10),
        };
        let code = AuthorizationCode::new(
            state.config.secrets.authorization_code_key.as_bytes(),
            code_payload.clone(),
        );
        let response = on_token_grant(
            Form(Request::AuthorizationCode {
                client_id: google_config.client_id.clone(),
                client_secret: google_config.client_secret.clone(),
                code: code.to_string(),
            }),
            state.token_store.clone(),
            state.config.clone(),
        )
        .await
        .unwrap()
        .into_inner();

        let response = on_token_grant(
            Form(Request::RefreshToken {
                client_id: google_config.client_id.clone(),
                client_secret: google_config.client_secret.clone(),
                refresh_token: response
                    .refresh_token
                    .expect("authorization grant did not return refresh token"),
            }),
            state.token_store.clone(),
            state.config.clone(),
        )
        .await
        .unwrap()
        .into_inner();

        let at = AccessToken::decode(
            state.config.secrets.access_key.as_bytes(),
            &response.access_token,
        )
        .unwrap();
        assert_eq!(at.sub, code_payload.sub);
    }

    mod refresh_token_grant {
        use super::*;

        #[actix_rt::test]
        async fn valid() {
            let state = get_state();
            let google_config = state.config.google.as_ref().unwrap();
            let refresh_token_payload = RefreshTokenPayload {
                sub: rand::random(),
                exp: Some(Utc::now() + Duration::minutes(10)),
                tid: rand::random(),
            };
            let refresh_token = RefreshToken::new(
                state.config.secrets.refresh_key.as_bytes(),
                refresh_token_payload.clone(),
            );
            state
                .token_store
                .add(&refresh_token.tid, refresh_token.exp.as_ref())
                .await
                .unwrap();
            let response = on_token_grant(
                Form(Request::RefreshToken {
                    client_id: google_config.client_id.clone(),
                    client_secret: google_config.client_secret.clone(),
                    refresh_token: refresh_token.to_string(),
                }),
                state.token_store.clone(),
                state.config.clone(),
            )
            .await
            .unwrap()
            .into_inner();

            let at = AccessToken::decode(
                state.config.secrets.access_key.as_bytes(),
                &response.access_token,
            )
            .unwrap();
            assert_eq!(at.sub, refresh_token_payload.sub);
        }

        #[actix_rt::test]
        async fn invalid_client() {
            let state = get_state();
            let response = on_token_grant(
                Form(Request::RefreshToken {
                    client_id: String::from("invalid-client-id"),
                    client_secret: String::from("invalid-client-secret"),
                    refresh_token: String::from("some-invalid-token"),
                }),
                state.token_store,
                state.config,
            )
            .await
            .unwrap_err();
            assert!(matches!(response, ResponseError::InvalidClient(..)))
        }
    }

    mod authorization_code_grant {
        use houseflow_types::token::AuthorizationCodePayload;

        use super::*;

        #[actix_rt::test]
        async fn valid() {
            let state = get_state();
            let google_config = state.config.google.as_ref().unwrap();
            let code_payload = AuthorizationCodePayload {
                sub: rand::random(),
                exp: Utc::now() + Duration::minutes(10),
            };
            let code = AuthorizationCode::new(
                state.config.secrets.authorization_code_key.as_bytes(),
                code_payload.clone(),
            );
            let response = on_token_grant(
                Form(Request::AuthorizationCode {
                    client_id: google_config.client_id.clone(),
                    client_secret: google_config.client_secret.clone(),
                    code: code.to_string(),
                }),
                state.token_store.clone(),
                state.config.clone(),
            )
            .await
            .unwrap()
            .into_inner();

            let (at, rt) = (response.access_token, response.refresh_token.unwrap());
            let (at, rt) = (
                AccessToken::decode(state.config.secrets.access_key.as_bytes(), &at).unwrap(),
                RefreshToken::decode(state.config.secrets.refresh_key.as_bytes(), &rt).unwrap(),
            );
            assert_eq!(at.sub, code_payload.sub);
            assert_eq!(rt.sub, code_payload.sub);
            assert!(
                state.token_store.exists(&rt.tid).await.unwrap(),
                "returned refresh token not found in store"
            );
        }

        #[actix_rt::test]
        async fn invalid_client() {
            let state = get_state();
            let response = on_token_grant(
                Form(Request::AuthorizationCode {
                    client_id: String::from("invalid-client-id"),
                    client_secret: String::from("invalid-client-secret"),
                    code: String::from("some-invalid-token"),
                }),
                state.token_store,
                state.config,
            )
            .await
            .unwrap_err();
            assert!(matches!(response, ResponseError::InvalidClient(..)))
        }
    }
}
