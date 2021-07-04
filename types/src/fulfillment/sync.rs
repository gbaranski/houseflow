use crate::{token, Device};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {}

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
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseBody {
    pub devices: Vec<Device>,
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for ResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TokenError(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        crate::json_error_response(self.status_code(), self)
    }
}
