use crate::{lighthouse, token, DeviceID};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    pub device_id: DeviceID,

    #[serde(flatten)]
    pub frame: lighthouse::proto::execute::Frame,
}

pub type Response = Result<ResponseBody, ResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum ResponseError {
    #[error("internal error: {0}")]
    InternalError(#[from] crate::InternalServerError),

    #[error("token error: {0}")]
    TokenError(#[from] token::Error),

    #[error("no device permission")]
    NoDevicePermission,

    #[error("Device is not connected")]
    DeviceNotConnected,

    #[error("error with device communication: {0}")]
    DeviceCommunicationError(#[from] lighthouse::DeviceCommunicationError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseBody {
    pub frame: lighthouse::proto::execute_response::Frame,
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for ResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TokenError(_) => StatusCode::UNAUTHORIZED,
            Self::NoDevicePermission => StatusCode::UNAUTHORIZED,
            Self::DeviceNotConnected => StatusCode::NOT_FOUND,
            Self::DeviceCommunicationError(_) => StatusCode::BAD_GATEWAY,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        crate::json_error_response(self.status_code(), self)
    }
}
