use crate::{lighthouse, token, DeviceID};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    pub device_id: DeviceID,

    #[serde(flatten)]
    pub frame: lighthouse::proto::execute::Frame,
}

pub type Response = Result<ResponseBody, ResponseError>;

#[houseflow_macros::server_error]
pub enum ResponseError {
    #[error("token error: {0}")]
    #[response(status_code = 401)]
    TokenError(#[from] token::Error),

    #[error("no device permission")]
    #[response(status_code = 401)]
    NoDevicePermission,

    #[error("Device is not connected")]
    #[response(status_code = 400)]
    DeviceNotConnected,

    #[error("error with device communication: {0}")]
    #[response(status_code = 502)]
    DeviceCommunicationError(#[from] lighthouse::DeviceCommunicationError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseBody {
    pub frame: lighthouse::proto::execute_response::Frame,
}
