use crate::token;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {}

pub type Response = Result<ResponseBody, ResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseBody {
    pub username: String,
    pub email: String,
}

#[houseflow_macros::server_error]
pub enum ResponseError {
    #[error("token error: {0}")]
    #[response(status_code = 401)]
    TokenError(#[from] token::Error),

    #[error("token not found in store")]
    #[response(status_code = 401)]
    TokenNotInStore,

    #[error("user not found")]
    #[response(status_code = 401)]
    UserNotFound,
}
