use crate::{DeviceTrait, DeviceType, ResultTagged, RoomID};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct AddDeviceRequest {
    pub room_id: RoomID,

    #[validate(length(min = 8))]
    pub password: String,
    pub device_type: DeviceType,
    pub traits: Vec<DeviceTrait>,
    pub name: String,
    pub will_push_state: bool,
    pub model: String,
    pub hw_version: Version,
    pub sw_version: Version,
    pub attributes: HashMap<String, Option<String>>,
}

pub type AddDeviceResponse = ResultTagged<AddDeviceResponseBody, AddDeviceResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum AddDeviceResponseError {
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

impl From<validator::ValidationErrors> for AddDeviceResponseError {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddDeviceResponseBody {}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for AddDeviceResponseError {
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
        let response = AddDeviceResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
