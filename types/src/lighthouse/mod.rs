use crate::accessory;
use crate::accessory::Accessory;
use serde::Deserialize;
use serde::Serialize;

pub type FrameID = u16;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ServerFrame {
    #[serde(rename = "hub/query")]
    HubQuery(HubQueryFrame),
    #[serde(rename = "accessory/execute")]
    AccessoryExecute(AccessoryExecuteFrame),
    #[serde(rename = "accessory/query")]
    AccessoryQuery(AccessoryQueryFrame),
}

impl ServerFrame {
    pub fn name(&self) -> &'static str {
        match self {
            Self::HubQuery(_) => "hub/query",
            Self::AccessoryExecute(_) => "accessory/execute",
            Self::AccessoryQuery(_) => "accessory/query",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum HubFrame {
    #[serde(rename = "hub/update")]
    HubUpdate(HubUpdateFrame),
    #[serde(rename = "accessory/execute-result")]
    AccessoryExecuteResult(AccessoryExecuteResultFrame),
    #[serde(rename = "accessory/update")]
    AccessoryUpdate(AccessoryUpdateFrame),
}

impl HubFrame {
    pub fn name(&self) -> &'static str {
        match self {
            Self::HubUpdate(_) => "hub/update",
            Self::AccessoryExecuteResult(_) => "accessory/execute-result",
            Self::AccessoryUpdate(_) => "accessory/update",
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HubQueryFrame {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct HubUpdateFrame {
    pub accessories: Vec<Accessory>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccessoryExecuteFrame {
    pub id: FrameID,
    #[serde(flatten)]
    pub command: accessory::Command,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessoryExecuteResultFrame {
    pub id: FrameID,
    #[serde(flatten)]
    pub status: accessory::Status,
    #[serde(default)]
    pub state: accessory::State,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccessoryQueryFrame {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessoryUpdateFrame {
    #[serde(default)]
    pub state: accessory::State,
}
