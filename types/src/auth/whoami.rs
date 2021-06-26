use serde::{Deserialize, Serialize};
use crate::{ResultTagged, token};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WhoamiRequest {}

pub type WhoamiResponse = ResultTagged<WhoamiResponseBody, WhoamiResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WhoamiResponseBody {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum WhoamiResponseError {
    #[error("decode token header error: {0}")]
    DecodeHeaderError(#[from] token::DecodeHeaderError),

    #[error("verify token error: {0}")]
    VerifyError(#[from] token::VerifyError),

    #[error("token not found in store")]
    TokenNotInStore,

    #[error("user not found")]
    UserNotFound,

    #[error("internal error: `{0}`")]
    InternalError(String),
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for WhoamiResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::UserNotFound => StatusCode::NOT_FOUND,
            Self::DecodeHeaderError(_) => StatusCode::BAD_REQUEST,
            Self::VerifyError(_) => StatusCode::UNAUTHORIZED,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TokenNotInStore => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = WhoamiResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
