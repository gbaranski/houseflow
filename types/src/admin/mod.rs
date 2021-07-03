pub mod device;
pub mod room;
pub mod structure;
pub mod user_structure;

use serde::{Serialize, Deserialize};
use crate::token;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum AddResponseError {
    #[error("internal error: `{0}`")]
    // Replace it with better type if needed
    InternalError(String),
    
    #[error("token error: {0}")]
    TokenError(#[from] token::Error),

    #[error("{0}")]
    ValidationError(#[from] validator::ValidationError),

    #[error("Device already exists")]
    DeviceAlreadyExists,

    #[error("User is not admin")]
    UserNotAdmin,
}

impl From<validator::ValidationErrors> for AddResponseError {
    fn from(val: validator::ValidationErrors) -> Self {
        Self::ValidationError(
            val.field_errors()
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
        let json = actix_web::web::Json(self.clone());
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
