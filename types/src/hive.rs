use serde::Deserialize;
use serde::Serialize;
use crate::accessory;
use crate::accessory::characteristics::Characteristic;
use crate::accessory::services::ServiceDiscriminants;
use crate::accessory::characteristics::CharacteristicDiscriminants;

pub type FrameID = u16;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum HubFrame {
    CharacteristicRead(CharacteristicRead),
    CharacteristicWrite(CharacteristicWrite),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum AccessoryFrame {
    CharacteristicUpdate(CharacteristicUpdate),
    CharacteristicReadResponse(CharacteristicReadResponse),
    CharacteristicWriteResponse(CharacteristicWriteResponse),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacteristicUpdate {
    pub service_name: ServiceDiscriminants,
    pub characteristic: Characteristic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacteristicRead {
    pub id: FrameID,
    pub service_name: ServiceDiscriminants,
    pub characteristic_name: CharacteristicDiscriminants,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacteristicWrite {
    pub id: FrameID,
    pub service_name: ServiceDiscriminants,
    pub characteristic: Characteristic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacteristicReadResponse {
    pub id: FrameID,
    pub result: accessory::Result<Characteristic>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacteristicWriteResponse {
    pub id: FrameID,
    pub result: accessory::Result<()>,
}