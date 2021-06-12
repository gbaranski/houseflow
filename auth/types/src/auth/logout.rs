use crate::ResultTagged;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogoutRequest {}

pub type LogoutResponse = ResultTagged<LogoutResponseBody, LogoutResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogoutResponseBody {
    pub token_removed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum LogoutResponseError {
    #[error("invalid token: {0}")]
    InvalidToken(#[from] token::VerifyError),

    #[error("invalid token: {0}")]
    DecodeHeaderError(#[from] token::DecodeHeaderError),
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for LogoutResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InvalidToken(_) => StatusCode::BAD_REQUEST,
            Self::DecodeHeaderError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = LogoutResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
