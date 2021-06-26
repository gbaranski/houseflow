use crate::{token, Device, ResultTagged};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncRequest {}

pub type SyncResponse = ResultTagged<SyncResponseBody, SyncResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum SyncResponseError {
    #[error("internal error: `{0}`")]
    // Replace it with better type if needed
    InternalError(String),

    #[error("decode token header error: {0}")]
    DecodeTokenHeaderError(#[from] token::DecodeHeaderError),

    #[error("token verify error: {0}")]
    TokenVerifyError(#[from] token::VerifyError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncResponseBody {
    pub devices: Vec<Device>,
}

#[cfg(feature = "db")]
impl From<db::Error> for SyncResponseError {
    fn from(v: db::Error) -> Self {
        Self::InternalError(v.to_string())
    }
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for SyncResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DecodeTokenHeaderError(_) => StatusCode::BAD_REQUEST,
            Self::TokenVerifyError(_) => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = SyncResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
