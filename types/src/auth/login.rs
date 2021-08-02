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
