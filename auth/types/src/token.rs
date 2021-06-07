use token::Token;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    RefreshToken,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessTokenRequest {
    /// The grant_type parameter must be set to `GrantType::RefreshToken`.
    pub grant_type: GrantType,

    /// The refresh token previously issued to the client.
    pub refresh_token: Token,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TokenType {
    Bearer,
}

pub type AccessTokenResponse = Result<AccessTokenResponseBody, AccessTokenError>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessTokenResponseBody {
    /// The access token string as issued by the authorization server.
    pub access_token: Token,

    /// The type of token this is, typically just the string “Bearer”.
    pub token_type: TokenType,

    /// If the access token expires, the server should reply with the duration of time the access token is granted for.
    #[serde(with = "token_expiration")]
    pub expires_in: Option<Duration>,
}

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[error("kind: `{error}`, description: `{}`")]
pub struct AccessTokenError {
    pub error: AccessTokenErrorKind,
    pub error_description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum AccessTokenErrorKind {
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
impl actix_web::ResponseError for AccessTokenError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;
        use AccessTokenErrorKind::*;

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

mod token_expiration {
    use super::*;
    use serde::{
        de::{self, Visitor},
        ser,
    };
    pub struct TokenExpirationVisitor;

    impl<'de> Visitor<'de> for TokenExpirationVisitor {
        type Value = Option<Duration>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("duration in seconds")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(Duration::from_secs(value)))
        }

        fn visit_some<D>(self, d: D) -> Result<Option<Duration>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            d.deserialize_i64(Self)
        }

        fn visit_none<E>(self) -> Result<Option<Duration>, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
        fn visit_unit<E>(self) -> Result<Option<Duration>, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *duration {
            Some(duration) => serializer.serialize_some(&duration.as_secs()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Option<Duration>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(d.deserialize_option(TokenExpirationVisitor)?)
    }
}
