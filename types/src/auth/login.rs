use crate::token::Token;
use crate::{ResultTagged, UserAgent};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
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

    #[error("validation error: {0}")]
    ValidationError(#[from] validator::ValidationError),

    #[error("internal error: `{0}`")]
    // Replace it with better type if needed
    InternalError(String),
}

impl From<validator::ValidationErrors> for LoginResponseError {
    fn from(val: validator::ValidationErrors) -> Self {
        Self::ValidationError(
            val.field_errors()
                .iter()
                .next()
                .unwrap()
                .1
                .first()
                .unwrap()
                .clone(),
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginResponseBody {
    pub refresh_token: Token,
    pub access_token: Token,
}

#[cfg(feature = "db")]
impl From<houseflow_db::Error> for LoginResponseError {
    fn from(v: houseflow_db::Error) -> Self {
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
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = LoginResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
