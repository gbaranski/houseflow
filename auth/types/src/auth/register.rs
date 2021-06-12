use types::ResultTagged;
use serde::{Deserialize, Serialize};
use types::UserID;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,

    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,
}

pub type RegisterResponse = ResultTagged<RegisterResponseBody, RegisterResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum RegisterResponseError {
    #[error("internal error: `{0}`")]
    // Replace it with better type if needed
    InternalError(String),

    #[error("{0}")]
    ValidationError(#[from] validator::ValidationError),

    #[error("User already exists")]
    UserAlreadyExists,
}

impl From<validator::ValidationErrors> for RegisterResponseError {
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
            Self::UserAlreadyExists => StatusCode::BAD_REQUEST,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let response = RegisterResponse::Err(self.clone());
        let json = actix_web::web::Json(response);
        actix_web::HttpResponse::build(self.status_code()).json(json)
    }
}
