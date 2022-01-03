use crate::accessory;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    pub device_id: accessory::ID,
    pub command: accessory::Command,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub status: accessory::Status,
    pub state: serde_json::Map<String, serde_json::Value>,
}
