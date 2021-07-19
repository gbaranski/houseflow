use crate::common::Credential;
use semver::Version;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

pub type DeviceID = Credential<16>;
pub type DevicePassword = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    /// Identifier of the device
    pub id: DeviceID,

    /// Name of room(if available)
    pub room_id: RoomID,

    /// Hashed password for device
    #[serde(skip)]
    pub password_hash: Option<String>,

    /// Type of the device
    pub device_type: DeviceType,

    /// Functionatily that the device has
    pub traits: Vec<DeviceTrait>,

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
    pub attributes: serde_json::Map<String, serde_json::Value>,
}

pub type StructureID = Credential<16>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Structure {
    pub id: StructureID,
    pub name: String,
}

pub type RoomID = Credential<16>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Room {
    pub id: RoomID,
    pub structure_id: StructureID,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserStructure {
    pub structure_id: StructureID,
    pub user_id: crate::UserID,
    pub is_manager: bool,
}

use strum::EnumString;

/// Traits defines what functionality device supports
#[derive(
    Debug, Clone, Hash, Eq, PartialEq, strum::Display, EnumString, EnumIter, Serialize, Deserialize,
)]
#[non_exhaustive]
pub enum DeviceTrait {
    OnOff,
    OpenClose,
}

impl DeviceTrait {
    pub fn commands(&self) -> Vec<DeviceCommand> {
        match *self {
            Self::OnOff => vec![DeviceCommand::OnOff],
            Self::OpenClose => vec![DeviceCommand::OpenClose],
        }
    }

    pub fn variants_string() -> Vec<String> {
        Self::iter().map(|e| e.to_string()).collect()
    }
}

/// Type of the device
#[derive(
    Debug, Clone, PartialEq, Eq, strum::Display, EnumString, EnumIter, Serialize, Deserialize,
)]
#[non_exhaustive]
pub enum DeviceType {
    Gate,
    Garage,
    Light,
}

impl DeviceType {
    pub fn required_traits(&self) -> Vec<DeviceTrait> {
        match *self {
            Self::Gate => vec![DeviceTrait::OpenClose],
            Self::Garage => vec![DeviceTrait::OpenClose],
            Self::Light => vec![DeviceTrait::OnOff],
        }
    }

    pub fn variants_string() -> Vec<String> {
        Self::iter().map(|e| e.to_string()).collect()
    }
}

#[derive(
    Debug, Clone, Eq, PartialEq, EnumIter, strum::Display, EnumString, Serialize, Deserialize,
)]
#[non_exhaustive]
pub enum DeviceCommand {
    OnOff,
    OpenClose,
}

impl DeviceCommand {
    pub fn variants_string() -> Vec<String> {
        Self::iter().map(|e| e.to_string()).collect()
    }
}

pub mod commands {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    pub struct OnOff {
        pub on: bool,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    pub struct OpenClose {
        #[serde(alias = "openPercent")]
        pub open_percent: u8,
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, strum::Display, EnumIter)]
#[non_exhaustive]
pub enum DeviceError {
    /// Actually, <device(s)> <doesn't/don't> support that functionality.
    FunctionNotSupported,

    /// Device does not support sent parameters
    InvalidParameters,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, strum::Display)]
#[serde(tag = "status", content = "description")]
#[repr(u8)]
pub enum DeviceStatus {
    /// Confirm that the command succeeded.
    Success,

    /// Target device is unable to perform the command.
    Error(DeviceError),
}

impl rand::distributions::Distribution<DeviceError> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> DeviceError {
        DeviceError::iter()
            .nth(rng.gen_range(0..DeviceError::iter().len()))
            .unwrap()
    }
}

impl rand::distributions::Distribution<DeviceCommand> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> DeviceCommand {
        DeviceCommand::iter()
            .nth(rng.gen_range(0..DeviceCommand::iter().len()))
            .unwrap()
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::ToSql for DeviceType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Text(self.to_string()),
        ))
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::ToSql for DeviceTrait {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Text(self.to_string()),
        ))
    }
}

#[cfg(feature = "rusqlite")]
use std::str::FromStr;

#[cfg(feature = "rusqlite")]
impl rusqlite::types::FromSql for DeviceType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Self::from_str(value.as_str()?)
            .map_err(|err| rusqlite::types::FromSqlError::Other(Box::new(err)))
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::types::FromSql for DeviceTrait {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        Self::from_str(value.as_str()?)
            .map_err(|err| rusqlite::types::FromSqlError::Other(Box::new(err)))
    }
}
