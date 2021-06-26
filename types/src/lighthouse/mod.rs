pub mod proto;

use crate::{DeviceID, ResultTagged};
use proto::{execute, execute_response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecuteRequest {
    pub device_id: DeviceID,
    pub frame: execute::Frame,
}

pub type ExecuteResponse = ResultTagged<ExecuteResponseBody, DeviceCommunicationError>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecuteResponseBody {
    pub frame: execute_response::Frame,
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for DeviceCommunicationError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = ExecuteResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}

#[derive(Debug, thiserror::Error, Deserialize, Serialize, Clone)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum DeviceCommunicationError {
    #[error("Timeout when sending request to device")]
    Timeout,
}

#[derive(Debug, thiserror::Error, Deserialize, Serialize, Clone)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum ConnectResponseError {
    #[error("internal error: {0}")]
    InternalError(String),

    #[error("invalid authorization header: {0}")]
    InvalidAuthorizationHeader(String),

    #[error("invalid credentials")]
    InvalidCredentials,
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for ConnectResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InvalidAuthorizationHeader(_) => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let json = actix_web::web::Json(self);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
