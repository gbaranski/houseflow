use chrono::Duration;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Request {
    /// The grant_type parameter must be set to `GrantType::RefreshToken`.
    pub grant_type: GrantType,

    /// The refresh token previously issued to the client.
    pub refresh_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    RefreshToken,
}

pub type Response = Result<ResponseBody, ResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct ResponseBody {
    /// The access token string as issued by the authorization server.
    pub access_token: String,

    /// The refresh token string as issued by the authorization server.
    pub refresh_token: Option<String>,

    /// The type of token this is, typically just the string “Bearer”.
    pub token_type: TokenType,

    /// If the access token expires, the server should reply with the duration of time the access token is granted for.
    #[serde(with = "token_expiration")]
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
    InternalError(#[from] crate::InternalServerError),

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

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            use std::convert::TryInto;
            let v: i64 = v.try_into().map_err(|err| {
                serde::de::Error::custom(&format!("u64 to i64 cast fail: {}", err))
            })?;
            self.visit_i64(v)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(Duration::seconds(value)))
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
            Some(duration) => serializer.serialize_some(&duration.num_seconds()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Option<Duration>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_option(TokenExpirationVisitor)
    }
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for ResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        crate::json_error_response(self.status_code(), self)
    }
}
