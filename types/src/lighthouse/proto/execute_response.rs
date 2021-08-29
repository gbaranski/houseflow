use crate::lighthouse::proto::FrameID;
use crate::DeviceStatus;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub id: FrameID,

    #[serde(flatten)]
    pub status: DeviceStatus,

    #[serde(default)]
    pub state: serde_json::Map<String, serde_json::Value>,
}
