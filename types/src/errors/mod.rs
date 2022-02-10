mod auth;
mod controller;
mod internal;
mod oauth;
mod provider;
mod token;

pub use auth::Error as AuthError;
pub use controller::Error as ControllerError;
pub use internal::Error as InternalError;
pub use oauth::Error as OAuthError;
pub use provider::Error as ProviderError;
pub use token::Error as TokenError;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error-description",
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
    #[error("controller error: {0}")]
    ControllerError(#[from] ControllerError),
    #[error("provider error: {0}")]
    ProviderError(#[from] ProviderError),
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        use crate::accessory;
        use axum::http::StatusCode;

        let status = match self {
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AuthError(ref err) => match err {
                AuthError::InvalidAuthorizationHeader(_) => StatusCode::UNAUTHORIZED,
                AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
                AuthError::InvalidPassword => StatusCode::UNAUTHORIZED,
                AuthError::UserNotFound => StatusCode::BAD_REQUEST,
                AuthError::HubNotFound => StatusCode::BAD_REQUEST,
                AuthError::UserAlreadyExists => StatusCode::NOT_ACCEPTABLE,
                AuthError::RefreshTokenBlacklisted => StatusCode::UNAUTHORIZED,
                AuthError::NoStructurePermission => StatusCode::UNAUTHORIZED,
                AuthError::InvalidVerificationCode(_) => StatusCode::UNAUTHORIZED,
                AuthError::NoAccessoryPermission => StatusCode::UNAUTHORIZED,
                AuthError::InvalidGoogleJwt(_) => StatusCode::UNAUTHORIZED,
                AuthError::InvalidCsrfToken => StatusCode::UNAUTHORIZED,
            },
            Self::OAuthError(_) => StatusCode::BAD_REQUEST,
            Self::ControllerError(ref err) => match err {
                ControllerError::AccessoryNotConnected => StatusCode::NOT_ACCEPTABLE,
                ControllerError::Timeout => StatusCode::REQUEST_TIMEOUT,
                ControllerError::AccessoryError(err) => match err {
                    accessory::Error::CharacteristicReadOnly => StatusCode::BAD_REQUEST,
                    accessory::Error::CharacteristicNotSupported => StatusCode::BAD_REQUEST,
                    accessory::Error::ServiceNotSupported => StatusCode::BAD_REQUEST,
                    accessory::Error::NotConnected => StatusCode::SERVICE_UNAVAILABLE,
                },
            },
            Self::ProviderError(ref err) => match err {
                ProviderError::AlreadyConnected => StatusCode::NOT_ACCEPTABLE,
            },
        };
        let mut response = axum::Json(self).into_response();
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
