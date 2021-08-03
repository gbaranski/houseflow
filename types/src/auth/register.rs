use crate::UserID;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Request {
    #[validate(email)]
    pub email: String,

    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,
}

pub type Response = Result<ResponseBody, ResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct ResponseBody {
    pub user_id: UserID,
}

#[houseflow_macros::server_error]
pub enum ResponseError {
    #[error("user already exists")]
    #[response(status_code = 400)]
    UserAlreadyExists,
}
