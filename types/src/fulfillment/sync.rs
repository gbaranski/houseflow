use crate::{token, Device};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {}

pub type Response = Result<ResponseBody, ResponseError>;

#[houseflow_macros::server_error]
pub enum ResponseError {
    #[error("token error: {0}")]
    #[response(status_code = 401)]
    TokenError(#[from] token::Error),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseBody {
    pub devices: Vec<Device>,
}
