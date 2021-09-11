use super::FrameID;
use crate::device;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub id: FrameID,
    pub command: device::Command,
}
