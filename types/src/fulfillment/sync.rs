use crate::device::Device;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub devices: Vec<Device>,
}
