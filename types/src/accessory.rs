use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

pub type ID = Uuid;
pub type Password = String;
pub type PasswordHash = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Accessory {
    /// ID of the accessory
    pub id: ID,
    /// Name of the accessory
    pub name: String,
    /// Name of the room that the accessory is in
    pub room_name: String,
    /// Type of the accessory, possibly with additional parameters
    #[serde(flatten)]
    pub r#type: Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "manufacturer", rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Type {
    XiaomiMijia(manufacturers::XiaomiMijia),
    Houseflow(manufacturers::Houseflow),
}

pub mod manufacturers {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(tag = "model", rename_all = "kebab-case")]
    #[non_exhaustive]
    pub enum XiaomiMijia {
        HygroThermometer,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(tag = "model", rename_all = "kebab-case")]
    #[non_exhaustive]
    pub enum Houseflow {
        Gate,
        Garage,
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, strum::Display)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Error {
    CharacteristicReadOnly,
    /// Accessory service does not support the specified characteristic
    CharacteristicNotSupported,
    /// Accessory does not support the specified service
    ServiceNotSupported,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, strum::Display)]
#[serde(tag = "status", content = "description")]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum Result<T> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value
    Err(Error),
}

impl<T> From<std::result::Result<T, Error>> for Result<T> {
    fn from(value: std::result::Result<T, Error>) -> Self {
        match value {
            Ok(value) => Self::Ok(value),
            Err(error) => Self::Err(error),
        }
    }
}

impl<T> Into<std::result::Result<T, Error>> for Result<T> {
    fn into(self) -> std::result::Result<T, Error> {
        match self {
            Self::Ok(value) => Ok(value),
            Self::Err(error) => Err(error),
        }
    }
}

pub mod services {
    use super::characteristics;
    use serde::Deserialize;
    use serde::Serialize;
    use strum::EnumDiscriminants;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumDiscriminants)]
    #[strum_discriminants(derive(Hash, Serialize, Deserialize, strum::Display))]
    #[strum_discriminants(name(ServiceName))]
    #[serde(tag = "name", rename_all = "kebab-case")]
    pub enum Service {
        TemperatureSensor(TemperatureSensor),
        HumiditySensor(HumiditySensor),
        GarageDoorOpener(GarageDoorOpener),
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TemperatureSensor {
        pub current_temperature: characteristics::CurrentTemperature,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct HumiditySensor {
        pub current_humidity: characteristics::CurrentHumidity,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct GarageDoorOpener {
        pub current_door_state: characteristics::CurrentDoorState,
        pub target_door_state: characteristics::TargetDoorState,
    }
}

pub mod characteristics {
    use serde::Deserialize;
    use serde::Serialize;
    use strum::EnumDiscriminants;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumDiscriminants)]
    #[strum_discriminants(derive(Serialize, Deserialize, strum::Display))]
    #[serde(tag = "name", rename_all = "kebab-case")]
    pub enum Characteristic {
        CurrentTemperature(CurrentTemperature),
        CurrentHumidity(CurrentHumidity),
        CurrentDoorState(CurrentDoorState),
        TargetDoorState(TargetDoorState),
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CurrentTemperature {
        pub temperature: f32,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CurrentHumidity {
        pub humidity: f32,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CurrentDoorState {
        pub open_percent: u8,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct TargetDoorState {
        pub open_percent: u8,
    }
}
