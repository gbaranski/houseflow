use serde::Deserialize;
use serde::Serialize;
use crate::accessory;

pub type FrameID = u16;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum HubFrame {
    Query(QueryFrame),
    Execute(ExecuteFrame),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum AccessoryFrame {
    State(StateFrame),
    ExecuteResult(ExecuteResultFrame),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QueryFrame {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecuteFrame {
    pub id: FrameID,
    #[serde(flatten)]
    pub command: accessory::Command,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteResultFrame {
    pub id: FrameID,
    #[serde(flatten)]
    pub status: accessory::Status,
    #[serde(default)]
    pub state: accessory::State,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateFrame {
    pub state: accessory::State,
}
