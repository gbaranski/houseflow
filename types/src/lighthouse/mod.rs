pub mod proto;

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum DeviceCommunicationError {
    #[error("internal error: {0}")]
    InternalError(String),

    #[error("Timeout when sending request to device")]
    Timeout,

    #[error("invalid JSON input")]
    InvalidJSON(String),
}

impl From<serde_json::Error> for DeviceCommunicationError {
    fn from(val: serde_json::Error) -> Self {
        Self::InvalidJSON(val.to_string())
    }
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

    #[error("handshake error: {0}")]
    HandshakeError(String),

    #[error("invalid authorization header: {0}")]
    InvalidAuthorizationHeader(String),

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("there is an existing connection with device with the specified ID")]
    AlreadyConnected,
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for ConnectResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InvalidAuthorizationHeader(_) => StatusCode::BAD_REQUEST,
            Self::HandshakeError(_) => StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AlreadyConnected => StatusCode::NOT_ACCEPTABLE,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let json = actix_web::web::Json(self);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
