use crate::ResultTagged;
use serde::{Deserialize, Serialize};
use token::Token;
use types::UserAgent;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub user_agent: UserAgent,
}

pub type LoginResponse = ResultTagged<LoginResponseBody, LoginResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum LoginResponseError {
    #[error("invalid password")]
    InvalidPassword,

    #[error("user not found")]
    UserNotFound,

    #[error("internal error: `{0}`")]
    // Replace it with better type if needed
    InternalError(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginResponseBody {
    pub refresh_token: Token,
    pub access_token: Token,
}

#[cfg(feature = "db")]
impl From<db::Error> for LoginResponseError {
    fn from(v: db::Error) -> Self {
        Self::InternalError(v.to_string())
    }
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for LoginResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InvalidPassword => StatusCode::BAD_REQUEST,
            Self::UserNotFound => StatusCode::NOT_FOUND,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = LoginResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
