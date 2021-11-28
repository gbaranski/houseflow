use crate::accessory;
use crate::structure;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    pub structure_id: structure::ID,
    pub device_id: accessory::ID,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub state: serde_json::Map<String, serde_json::Value>,
}
