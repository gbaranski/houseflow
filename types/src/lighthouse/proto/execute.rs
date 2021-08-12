use crate::lighthouse::proto::FrameID;
use crate::DeviceCommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub id: FrameID,
    pub command: DeviceCommand,
    pub params: serde_json::Map<String, serde_json::Value>,
}
