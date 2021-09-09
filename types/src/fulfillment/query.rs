use crate::device;
use crate::lighthouse;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    pub device_id: device::ID,
    #[serde(flatten)]
    pub frame: lighthouse::query::Frame,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub frame: lighthouse::state::Frame,
}
