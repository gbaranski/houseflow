use houseflow_token::Token;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
// #[cfg_attr(feature = "serde", derive(serdeDeserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum GrantType {
    RefreshToken,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AccessTokenRequestBody {
    /// The grant_type parameter must be set to `GrantType::RefreshToken`.
    pub grant_type: GrantType,

    /// The refresh token previously issued to the client.
    pub refresh_token: Token,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TokenType {
    Bearer,
}

pub type AccessTokenResponse = Result<AccessTokenRequestBody, AccessTokenRequestError>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AccessTokenResponseBody {
    /// The access token string as issued by the authorization server.
    pub access_token: Token,

    /// The type of token this is, typically just the string “Bearer”.
    pub token_type: TokenType,

    /// If the access token expires, the server should reply with the duration of time the access token is granted for.
    pub expires_in: Option<Duration>,
}

#[derive(Debug, Clone, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[error("kind: `{error}`, description: `{}`")]
pub struct AccessTokenRequestError {
    pub error: AccessTokenRequestErrorKind,
    pub error_description: Option<String>,
}

#[derive(Debug, Clone, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum AccessTokenRequestErrorKind {
    /// The request is missing a parameter so the server can’t proceed with the request.
    /// This may also be returned if the request includes an unsupported parameter or repeats a parameter.
    #[error("Request is missing a parameter")]
    InvalidRequest,

    /// Client authentication failed, such as if the request contains an invalid client ID or secret.
    /// Send an HTTP 401 response in this case.
    #[error("Invalid Client ID or Secret")]
    InvalidClient,

    /// The authorization code (or user’s password for the password grant type) is invalid or expired.
    /// This is also the error you would return if the redirect URL given in the authorization grant does not match the URL provided in this access token request.
    #[error("Authorization Code(or user's password) is invalid, or redirect URL does not match")]
    InvalidGrant,

    /// For access token requests that include a scope (password or client_credentials grants), this error indicates an invalid scope value in the request.
    #[error("Scope of requested access token is invalid")]
    InvalidScope,

    /// This client is not authorized to use the requested grant type.
    /// For example, if you restrict which applications can use the Implicit grant, you would return this error for the other apps.
    #[error("Client is not authorized to use the requested grant type")]
    UnauthorizedClient,

    /// If a grant type is requested that the authorization server doesn’t recognize, use this code.
    /// Note that unknown grant types also use this specific error code rather than using the invalid_request above.
    #[error("Grant type is unsupported")]
    UnsupportedGrantType,
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for AccessTokenRequestError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;
        use AccessTokenRequestErrorKind::*;

        match self.error {
            InvalidRequest => StatusCode::BAD_REQUEST,
            InvalidClient => StatusCode::UNAUTHORIZED,
            InvalidGrant => StatusCode::BAD_REQUEST,
            InvalidScope => StatusCode::BAD_REQUEST,
            UnauthorizedClient => StatusCode::BAD_REQUEST,
            UnsupportedGrantType => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let json = actix_web::web::Json(self);

        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}

