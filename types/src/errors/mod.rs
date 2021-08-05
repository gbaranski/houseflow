mod auth;
mod fulfillment;
mod internal;
mod lighthouse;
mod token;

pub use auth::Error as AuthError;
pub use fulfillment::Error as FulfillmentError;
pub use internal::Error as InternalError;
pub use lighthouse::Error as LighthouseError;
pub use token::Error as TokenError;

use serde::{Deserialize, Serialize};

#[cfg(feature = "validator")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationError(validator::ValidationError);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum ServerError {
    #[error("internal error: {0}")]
    InternalError(#[from] InternalError),

    #[error("validation error: {0}")]
    ValidationError(#[from] ValidationError),

    #[error("auth error: {0}")]
    AuthError(#[from] AuthError),

    #[error("fulfillment error: {0}")]
    FulfillmentError(#[from] FulfillmentError),

    #[error("lighthouse error: {0}")]
    LighthouseError(#[from] LighthouseError),
}

#[cfg(feature = "axum")]
impl axum_crate::response::IntoResponse for ServerError {
    fn into_response(self) -> http::Response<axum_crate::body::Body> {
        use http::StatusCode;
        let status = match self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AuthError(ref err) => match err {
                AuthError::InvalidAuthorizationHeader(_) => StatusCode::UNAUTHORIZED,
                AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
                AuthError::InvalidPassword => StatusCode::UNAUTHORIZED,
                AuthError::UserNotFound => StatusCode::NOT_FOUND,
                AuthError::DeviceNotFound => StatusCode::NOT_FOUND,
                AuthError::UserAlreadyExists => StatusCode::NOT_ACCEPTABLE,
                AuthError::RefreshTokenNotInStore => StatusCode::UNAUTHORIZED,
                AuthError::NoDevicePermission => StatusCode::UNAUTHORIZED,
            },
            Self::FulfillmentError(ref err) => match err {
                FulfillmentError::DeviceNotConnected => StatusCode::BAD_GATEWAY,
                FulfillmentError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            },
            Self::LighthouseError(ref err) => match err {
                LighthouseError::AlreadyConnected => StatusCode::NOT_ACCEPTABLE,
            },
        };
        let response = axum_crate::response::Json(serde_json::to_string(&self).unwrap());
        let mut response = axum_crate::response::IntoResponse::into_response(response);
        *response.status_mut() = status;

        response
    }
}

#[cfg(feature = "validator")]
impl From<validator::ValidationErrors> for ValidationError {
    fn from(errors: validator::ValidationErrors) -> Self {
        Self(
            errors
                .field_errors()
                .iter()
                .next()
                .unwrap()
                .1
                .first()
                .unwrap()
                .clone(),
        )
    }
}

#[cfg(feature = "validator")]
impl From<validator::ValidationErrors> for ServerError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let validation_error = ValidationError::from(errors);
        Self::ValidationError(validation_error)
    }
}

#[cfg(feature = "validator")]
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "validator")]
impl std::error::Error for ValidationError {}
