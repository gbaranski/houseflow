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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum ResponseError {
    #[error("token error: {0}")]
    TokenError(#[from] token::Error),

    #[error("token not found in store")]
    TokenNotInStore,

    #[error("user not found")]
    UserNotFound,
}
