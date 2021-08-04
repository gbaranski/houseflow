use crate::Device;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub devices: Vec<Device>,
}
