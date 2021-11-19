mod auth;
mod fulfillment;
mod internal;
mod lighthouse;
mod oauth;
mod token;

pub use auth::Error as AuthError;
pub use fulfillment::Error as FulfillmentError;
pub use internal::Error as InternalError;
pub use lighthouse::Error as LighthouseError;
pub use oauth::Error as OAuthError;
pub use token::Error as TokenError;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum ServerError {
    #[error("internal error: {0}")]
    InternalError(#[from] InternalError),
    #[error("too many requests, please slow down")]
    TooManyRequests,
    #[error("validation error: {0}")]
    ValidationError(String),
    #[error("auth error: {0}")]
    AuthError(#[from] AuthError),
    #[error("oauth error: {0}")]
    OAuthError(#[from] OAuthError),
    #[error("fulfillment error: {0}")]
    FulfillmentError(#[from] FulfillmentError),
    #[error("lighthouse error: {0}")]
    LighthouseError(#[from] LighthouseError),
}

#[cfg(feature = "axum")]
impl axum_crate::response::IntoResponse for ServerError {
    type Body = http_body::Full<hyper::body::Bytes>;

    type BodyError = <Self::Body as axum_crate::body::HttpBody>::Error;

    fn into_response(self) -> http::Response<Self::Body> {
        use http::StatusCode;
        let status = match self {
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AuthError(ref err) => match err {
                AuthError::InvalidAuthorizationHeader(_) => StatusCode::UNAUTHORIZED,
                AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
                AuthError::InvalidPassword => StatusCode::UNAUTHORIZED,
                AuthError::UserNotFound => StatusCode::BAD_REQUEST,
                AuthError::DeviceNotFound => StatusCode::BAD_REQUEST,
                AuthError::UserAlreadyExists => StatusCode::NOT_ACCEPTABLE,
                AuthError::RefreshTokenBlacklisted => StatusCode::UNAUTHORIZED,
                AuthError::NoDevicePermission => StatusCode::UNAUTHORIZED,
                AuthError::InvalidVerificationCode(_) => StatusCode::UNAUTHORIZED,
                AuthError::InvalidGoogleJwt(_) => StatusCode::UNAUTHORIZED,
                AuthError::InvalidCsrfToken => StatusCode::UNAUTHORIZED,
            },
            Self::OAuthError(_) => StatusCode::BAD_REQUEST,
            Self::FulfillmentError(ref err) => match err {
                FulfillmentError::DeviceNotConnected => StatusCode::NOT_ACCEPTABLE,
                FulfillmentError::Timeout => StatusCode::REQUEST_TIMEOUT,
            },
            Self::LighthouseError(ref err) => match err {
                LighthouseError::AlreadyConnected => StatusCode::NOT_ACCEPTABLE,
            },
        };
        let mut response = axum_crate::Json(self).into_response();
        *response.status_mut() = status;

        response
    }
}

impl From<TokenError> for ServerError {
    fn from(e: TokenError) -> Self {
        Self::AuthError(e.into())
    }
}

#[cfg(feature = "validator")]
impl From<validator::ValidationErrors> for ServerError {
    fn from(errors: validator::ValidationErrors) -> Self {
        Self::ValidationError(errors.to_string())
    }
}

#[cfg(feature = "askama")]
impl From<askama::Error> for InternalError {
    fn from(e: askama::Error) -> Self {
        Self::Template(e.to_string())
    }
}

#[cfg(feature = "askama")]
impl From<askama::Error> for ServerError {
    fn from(e: askama::Error) -> Self {
        Self::InternalError(e.into())
    }
}
