use crate::lighthouse::proto::{query, state};
use crate::{lighthouse, token, DeviceID, ResultTagged};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryRequest {
    pub device_id: DeviceID,

    #[serde(flatten)]
    pub frame: query::Frame,
}

pub type QueryResponse = ResultTagged<QueryResponseBody, QueryResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum QueryResponseError {
    #[error("internal error: `{0}`")]
    // Replace it with better type if needed
    InternalError(String),

    #[error("decode token header error: {0}")]
    DecodeTokenHeaderError(#[from] token::DecodeHeaderError),

    #[error("token verify error: {0}")]
    TokenVerifyError(#[from] token::VerifyError),

    #[error("no device permission")]
    NoDevicePermission,

    #[error("Device is not connected")]
    DeviceNotConnected,

    #[error("error with device communication: {0}")]
    DeviceCommunicationError(#[from] lighthouse::DeviceCommunicationError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryResponseBody {
    pub frame: state::Frame,
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for QueryResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use crate::lighthouse::DeviceCommunicationError;
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DecodeTokenHeaderError(_) => StatusCode::BAD_REQUEST,
            Self::TokenVerifyError(_) => StatusCode::UNAUTHORIZED,
            Self::NoDevicePermission => StatusCode::FORBIDDEN,
            Self::DeviceNotConnected => StatusCode::NOT_FOUND,
            Self::DeviceCommunicationError(err) => match err {
                DeviceCommunicationError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            },
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = QueryResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
