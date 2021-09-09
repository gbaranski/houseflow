use crate::lighthouse;
use crate::device;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    pub device_id: device::ID,
    #[serde(flatten)]
    pub frame: lighthouse::execute::Frame,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub frame: lighthouse::execute_response::Frame,
}
