use crate::lighthouse::proto::FrameID;
use crate::{DeviceError, DeviceStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub id: FrameID,
    pub status: DeviceStatus,
    pub error: DeviceError,
    pub state: serde_json::Map<String, serde_json::Value>,
}
