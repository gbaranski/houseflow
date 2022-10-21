use crate::accessory;
use crate::accessory::characteristics::Characteristic;
use crate::accessory::characteristics::CharacteristicName;
use crate::accessory::services::ServiceName;
use crate::accessory::Accessory;
use serde::Deserialize;
use serde::Serialize;

pub type FrameID = u16;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Service {
    pub name: ServiceName
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EmptyCharacteristic {
    pub name: CharacteristicName
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ServerFrame {
    ReadCharacteristic{
        id: FrameID,
        #[serde(rename = "accessory-id")]
        accessory_id: accessory::ID,
        service: Service,
        characteristic: EmptyCharacteristic
    },
    WriteCharacteristic{
        id: FrameID,
        #[serde(rename = "accessory-id")]
        accessory_id: accessory::ID,
        service: Service,
        characteristic: Characteristic,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum HubFrame {
    AccessoryConnected{
        accessory: Accessory
    },
    AccessoryDisconnected{
        #[serde(rename = "accessory-id")]
        accessory_id: accessory::ID
    },
    UpdateCharacteristic{
        #[serde(rename = "accessory-id")]
        accessory_id: accessory::ID,
        #[serde(rename = "service-name")]
        service_name: ServiceName,
        characteristic: Characteristic,
    },
    ReadCharacteristicResult{
        id: FrameID,
        result: accessory::Result<Characteristic>,
    },
    WriteCharacteristicResult{
        id: FrameID,
        result: accessory::Result<()>,
    },
}
