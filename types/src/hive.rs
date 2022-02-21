use crate::accessory;
use crate::accessory::characteristics::Characteristic;
use crate::accessory::characteristics::CharacteristicName;
use crate::accessory::services::ServiceName;
use serde::Deserialize;
use serde::Serialize;

pub type FrameID = u16;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum HubFrame {
    ReadCharacteristic(ReadCharacteristic),
    WriteCharacteristic(WriteCharacteristic),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum AccessoryFrame {
    UpdateCharacteristic(UpdateCharacteristic),
    CharacteristicReadResult(CharacteristicReadResult),
    CharacteristicWriteResult(CharateristicWriteResult),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateCharacteristic {
    pub service_name: ServiceName,
    pub characteristic: Characteristic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadCharacteristic {
    pub id: FrameID,
    pub service_name: ServiceName,
    pub characteristic_name: CharacteristicName,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WriteCharacteristic {
    pub id: FrameID,
    pub service_name: ServiceName,
    pub characteristic: Characteristic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacteristicReadResult {
    pub id: FrameID,
    pub result: accessory::Result<Characteristic>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharateristicWriteResult {
    pub id: FrameID,
    pub result: accessory::Result<()>,
}
