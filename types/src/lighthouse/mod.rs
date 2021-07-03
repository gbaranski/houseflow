pub mod proto;

use serde::{Deserialize, Serialize};


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
