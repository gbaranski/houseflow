use crate::token;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Request {}

pub type Response = Result<ResponseBody, ResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct ResponseBody {}

#[houseflow_macros::server_error]
pub enum ResponseError {
    #[error("token error: {0}")]
    #[response(status_code = 403)]
    TokenError(#[from] token::Error),
}
