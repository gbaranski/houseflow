use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Request {
    pub refresh_token: String,
}

pub type Response = Result<ResponseBody, ResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct ResponseBody {
    pub refresh_token: Option<String>,

    pub access_token: String,
}

#[houseflow_macros::server_error]
pub enum ResponseError {
    #[error("token error: {0}")]
    #[response(status_code = 401)]
    TokenError(#[from] crate::token::DecodeError),

    #[error("token not found in store")]
    #[response(status_code = 401)]
    TokenNotInStore,
}
