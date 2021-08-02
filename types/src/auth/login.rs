use houseflow_macros::server_error;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Request {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}

pub type Response = Result<ResponseBody, ResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct ResponseBody {
    pub refresh_token: String,

    pub access_token: String,
}

#[server_error]
pub enum ResponseError {
    #[error("invalid password")]
    #[response(status_code = 400)]
    InvalidPassword,

    #[error("user not found")]
    #[response(status_code = 404)]
    UserNotFound,
}

// #[cfg(feature = "actix")]
// impl actix_web::ResponseError for ResponseError {
//     fn status_code(&self) -> actix_web::http::StatusCode {
//         use actix_web::http::StatusCode;
// 
//         match self {
//             Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
//             Self::ValidationError(_) => StatusCode::BAD_REQUEST,
//             Self::InvalidPassword => StatusCode::UNAUTHORIZED,
//             Self::UserNotFound => StatusCode::UNAUTHORIZED,
//         }
//     }
// 
//     fn error_response(&self) -> actix_web::HttpResponse {
//         crate::json_error_response(self.status_code(), self)
//     }
// }
