pub mod device;
pub mod room;
pub mod structure;
pub mod user_structure;

use crate::token;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum AddResponseError {
    #[error("internal error: {0}")]
    InternalError(#[from] crate::InternalServerError),

    #[error("validation error: {0}")]
    ValidationError(#[from] crate::ValidationError),

    #[error("token error: {0}")]
    TokenError(#[from] token::Error),

    #[error("Device already exists")]
    DeviceAlreadyExists,

    #[error("User is not admin")]
    UserNotAdmin,
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for AddResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::TokenError(err) => err.status_code(),
            Self::DeviceAlreadyExists => StatusCode::BAD_REQUEST,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::UserNotAdmin => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        crate::json_error_response(self.status_code(), self)
    }
}
