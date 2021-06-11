use crate::ResultTagged;
use serde::{Deserialize, Serialize};
use types::UserID;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub type RegisterResponse = ResultTagged<RegisterResponseBody, RegisterResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
pub enum RegisterResponseError {
    #[error("internal error: `{0}`")]
    // Replace it with better type if needed
    InternalError(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterResponseBody {
    pub user_id: UserID,
}

#[cfg(feature = "db")]
impl From<db::Error> for RegisterResponseError {
    fn from(v: db::Error) -> Self {
        Self::InternalError(v.to_string())
    }
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for RegisterResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = RegisterResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}

