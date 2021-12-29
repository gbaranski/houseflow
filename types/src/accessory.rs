use serde::Deserialize;
use serde::Serialize;
use strum::EnumVariantNames;
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

/// Traits defines what functionality accessory supports
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Trait {
    OnOff,
    OpenClose,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumVariantNames)]
#[non_exhaustive]
#[serde(tag = "command", content = "params", rename_all = "kebab-case")]
pub enum Command {
    OnOff(commands::OnOff),
    OpenClose(commands::OpenClose),
}

impl Command {
    pub fn is_supported(&self, t: &[Trait]) -> bool {
        let required_traits = match self {
            Command::OnOff(_) => &[Trait::OnOff],
            Command::OpenClose(_) => &[Trait::OpenClose],
        };

        for required_trait in required_traits {
            if !t.contains(required_trait) {
                return false;
            }
        }
        true
    }
}

pub mod commands {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    pub struct OnOff {
        pub on: bool,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    pub struct OpenClose {
        pub open_percent: u8,
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, strum::Display)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Error {
    /// Device does not support the specified command
    CommandNotSupported,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, strum::Display)]
#[serde(tag = "status", content = "description")]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    /// Confirm that the command succeeded.
    Success,
    /// Target device is unable to perform the command.
    Error(Error),
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct State {
    pub temperature: Option<f32>,
    pub humidity: Option<u8>,
    pub on: Option<bool>,
    pub open_percent: Option<u8>,
    pub battery_percent: Option<u16>,
    pub battery_voltage: Option<f32>,
}
