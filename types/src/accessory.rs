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

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Error {
    /// Accessory is not connected
    #[error("accessory is not connected")]
    NotConnected,
    #[error("characteristic is read only")]
    CharacteristicReadOnly,
    /// Accessory service does not support the specified characteristic
    #[error("characteristic is not supported")]
    CharacteristicNotSupported,
    /// Accessory does not support the specified service
    #[error("service is not supported")]
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

impl<T> From<Result<T>> for std::result::Result<T, Error> {
    fn from(value: Result<T>) -> std::result::Result<T, Error> {
        match value {
            Result::Ok(value) => Ok(value),
            Result::Err(error) => Err(error),
        }
    }
}

pub mod services {
    use super::characteristics;
    use serde::Deserialize;
    use serde::Serialize;
    use strum::EnumDiscriminants;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumDiscriminants)]
    #[strum_discriminants(derive(
        Hash,
        Serialize,
        Deserialize,
        strum::Display,
        strum::EnumVariantNames,
        strum::EnumString
    ))]
    #[strum_discriminants(name(ServiceName))]
    #[strum_discriminants(strum(serialize_all = "kebab-case"))]
    #[strum_discriminants(serde(rename_all = "kebab-case"))]
    #[serde(tag = "name", rename_all = "kebab-case")]
    #[strum(serialize_all = "kebab-case")]
    pub enum Service {
        TemperatureSensor(TemperatureSensor),
        HumiditySensor(HumiditySensor),
        GarageDoorOpener(GarageDoorOpener),
        Battery(Battery),
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Battery {
        pub battery_level: characteristics::BatteryLevel,
    }
}

pub mod characteristics {
    use serde::Deserialize;
    use serde::Serialize;
    use strum::EnumDiscriminants;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumDiscriminants)]
    #[strum_discriminants(derive(
        Serialize,
        Deserialize,
        strum::Display,
        strum::EnumVariantNames,
        strum::EnumString
    ))]
    #[strum_discriminants(name(CharacteristicName))]
    #[strum_discriminants(strum(serialize_all = "kebab-case"))]
    #[strum_discriminants(serde(rename_all = "kebab-case"))]
    #[serde(tag = "name", rename_all = "kebab-case")]
    #[strum(serialize_all = "kebab-case")]
    pub enum Characteristic {
        CurrentTemperature(CurrentTemperature),
        CurrentHumidity(CurrentHumidity),
        CurrentDoorState(CurrentDoorState),
        TargetDoorState(TargetDoorState),
        BatteryLevel(BatteryLevel),
        ChargingState(ChargingState),
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct CurrentTemperature {
        pub temperature: f32,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct CurrentHumidity {
        pub humidity: f32,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct CurrentDoorState {
        pub open_percent: u8,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct TargetDoorState {
        pub open_percent: u8,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct BatteryLevel {
        pub battery_level_percent: u8,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum ChargingState {
        NotCharging,
        Charging,
        NotChargeable,
    }
}
