use crate::room;
use semver::Version;
use serde::Deserialize;
use serde::Serialize;
use std::convert::TryFrom;
use strum::EnumString;
use strum::EnumVariantNames;
use strum::IntoStaticStr;
use uuid::Uuid;

pub type ID = Uuid;
pub type Password = String;
pub type PasswordHash = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Device {
    /// Identifier of the device
    pub id: ID,
    /// Name of room(if available)
    pub room_id: room::ID,
    /// Hashed password for device
    pub password_hash: Option<PasswordHash>,
    /// Type of the device
    #[serde(rename = "type")]
    pub device_type: Type,
    /// Functionatily that the device has
    pub traits: Vec<Trait>,
    /// Name of the device
    pub name: String,
    /// True if device will push state by itself, otherwise will use polling
    pub will_push_state: bool,
    /// The model or SKU identifier of the device
    pub model: String,
    /// Specific version number of hardware of the device
    pub hw_version: Version,
    /// Specific version number of software of the device
    pub sw_version: Version,
    /// Aligned with per-trait attributes described in each trait schema reference.
    #[serde(default)]
    pub attributes: serde_json::Map<String, serde_json::Value>,
}

/// Traits defines what functionality device supports
#[derive(
    Debug,
    Clone,
    Hash,
    Eq,
    PartialEq,
    strum::Display,
    IntoStaticStr,
    EnumString,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Trait {
    OnOff,
    OpenClose,
}

impl TryFrom<&str> for Trait {
    type Error = strum::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        std::str::FromStr::from_str(value)
    }
}

/// Type of the device
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    strum::Display,
    EnumString,
    EnumVariantNames,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Type {
    Garage,
    Gate,
    Light,
}

impl Type {
    pub fn required_traits(&self) -> Vec<Trait> {
        match *self {
            Self::Gate => vec![Trait::OpenClose],
            Self::Garage => vec![Trait::OpenClose],
            Self::Light => vec![Trait::OnOff],
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, strum::Display, EnumVariantNames, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "command", content = "params", rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
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
    /// Actually, <device(s)> <doesn't/don't> support that functionality.
    FunctionNotSupported,
    /// Device does not support sent parameters
    InvalidParameters,
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
