use crate::{ResultTagged, StructureID, UserID};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct AddUserStructureRequest {
    pub structure_id: StructureID,
    pub user_id: UserID,
    pub is_manager: bool,
}

pub type AddUserStructureResponse =
    ResultTagged<AddUserStructureResponseBody, AddUserStructureResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum AddUserStructureResponseError {
    #[error("internal error: `{0}`")]
    // Replace it with better type if needed
    InternalError(String),

    #[error("{0}")]
    ValidationError(#[from] validator::ValidationError),

    #[error("Device already exists")]
    DeviceAlreadyExists,

    #[error("invalid in-header token: {0}")]
    InvalidHeaderToken(#[from] crate::token::DecodeHeaderError),

    #[error("invalid token: {0}")]
    InvalidToken(#[from] crate::token::VerifyError),

    #[error("User is not admin")]
    UserNotAdmin,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddUserStructureResponseBody {}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for AddUserStructureResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InvalidHeaderToken(_) => StatusCode::BAD_REQUEST,
            Self::DeviceAlreadyExists => StatusCode::BAD_REQUEST,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::UserNotAdmin => StatusCode::FORBIDDEN,
            Self::InvalidToken(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = AddUserStructureResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
