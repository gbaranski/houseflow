use serde::{Deserialize, Serialize};
use types::{DeviceID, ResultTagged};
use lighthouse_proto::{execute, execute_response};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecuteRequest {
    pub device_id: DeviceID,

    #[serde(flatten)]
    pub frame: execute::Frame,
}

pub type ExecuteResponse = ResultTagged<ExecuteResponseBody, ExecuteResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum ExecuteResponseError {
    #[error("internal error: `{0}`")]
    // Replace it with better type if needed
    InternalError(String),

    #[error("decode token header error: {0}")]
    DecodeTokenHeaderError(#[from] token::DecodeHeaderError),

    #[error("token verify error: {0}")]
    TokenVerifyError(#[from] token::VerifyError),

    #[error("no device permission")]
    NoDevicePermission,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecuteResponseBody {
    pub frame: execute_response::Frame,
}

#[cfg(feature = "db")]
impl From<db::Error> for ExecuteResponseError {
    fn from(v: db::Error) -> Self {
        Self::InternalError(v.to_string())
    }
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for ExecuteResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DecodeTokenHeaderError(_) => StatusCode::BAD_REQUEST,
            Self::TokenVerifyError(_) => StatusCode::UNAUTHORIZED,
            Self::NoDevicePermission => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = ExecuteResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
