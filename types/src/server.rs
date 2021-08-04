use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum InternalServerError {
    #[error("token store error: {0}")]
    TokenStoreError(String),

    #[error("database error: {0}")]
    DatabaseError(String),

    #[error("other: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum FulfillmentError {
    #[error("device not connected")]
    DeviceNotConnected,

    #[error("user does not have permission to a specified device")]
    NoDevicePermission,
}


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
    InternalError(#[from] InternalServerError),

    #[error("validation error: {0}")]
    ValidationError(#[from] ValidationError),

    #[error("fulfillment error: {0}")]
    FulfillmentError(#[from] FulfillmentError),

    #[error("invalid authorization header {0}")]
    InvalidAuthorizationHeader(String),
    
    #[error("invalid error: {0}")]
    InvalidToken(#[from] crate::token::Error),

    #[error("connect error: {0}")]
    DeviceConnectError(#[from] crate::lighthouse::ConnectError),

    #[error("device communication error: {0}")]
    DeviceCommunicationError(#[from] crate::lighthouse::DeviceCommunicationError),

    /// When password hashes doesn't match with the one from database
    #[error("invalid password")]
    InvalidPassword,

    /// When client tries to log in, but user with given credentails have not been found
    #[error("user not found")]
    UserNotFound,

    /// Occurs when user tries to register, but user with given credentials already exists
    #[error("user already exists")]
    UserAlreadyExists,

    /// Refresh token not found in store, this may occur when refreshing access token
    #[error("token not found in store")]
    RefreshTokenNotInStore,

    /// User does not have access to device
    #[error("no device permission")]
    NoDevicePermission,
}

#[cfg(feature = "axum")]
impl axum_crate::response::IntoResponse for ServerError {
    fn into_response(self) -> http::Response<axum_crate::body::Body> {
        use http::StatusCode;
        let status = match self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => todo!(),
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
