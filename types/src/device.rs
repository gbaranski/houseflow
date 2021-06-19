use crate::common::Credential;
use semver::Version;
use std::collections::HashMap;
use strum::{EnumIter, IntoEnumIterator};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type DeviceID = Credential<16>;
pub type DevicePassword = String;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Device {
    /// Identifier of the device
    pub id: DeviceID,

    /// Hashed password for device
    pub password_hash: String,

    /// Type of the device
    pub device_type: DeviceType,

    /// Functionatily that the device has
    pub traits: Vec<DeviceTrait>,

    /// Name of the device
    pub name: String,

    /// True if device will push state by itself, otherwise will use polling
    pub will_push_state: bool,

    /// Name of room(if available)
    pub room: Option<String>,

    /// The model or SKU identifier of the device
    pub model: String,

    /// Specific version number of hardware of the device
    pub hw_version: Version,

    /// Specific version number of software of the device
    pub sw_version: Version,

    /// Aligned with per-trait attributes described in each trait schema reference.
    pub attributes: HashMap<String, Option<String>>,
}

/// User permission to a device
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DevicePermission {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

use strum::EnumString;

/// Traits defines what functionality device supports
#[derive(Debug, Clone, Hash, Eq, PartialEq, strum::Display, EnumString)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum DeviceTrait {
    OnOff,
    OpenClose,
}

impl DeviceTrait {
    pub fn commands(&self) -> Vec<DeviceCommand> {
        match *self {
            Self::OnOff => vec![DeviceCommand::NoOperation, DeviceCommand::OnOff],
            Self::OpenClose => vec![DeviceCommand::NoOperation, DeviceCommand::OpenClose],
        }
    }
}

/// Type of the device
#[derive(Debug, Clone, PartialEq, Eq, strum::Display, EnumString)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[repr(u16)]
pub enum DeviceType {
    Gate,
    Garage,
}

impl DeviceType {
    pub fn required_traits(&self) -> Vec<DeviceTrait> {
        match *self {
            Self::Gate => vec![DeviceTrait::OpenClose],
            Self::Garage => vec![DeviceTrait::OpenClose],
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, strum::Display, EnumString)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[strum(serialize_all = "snake_case")]
#[repr(u16)]
pub enum DeviceCommand {
    NoOperation = 0x0000,
    OnOff = 0x0001,
    OpenClose = 0x0002,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
#[repr(u16)]
pub enum DeviceError {
    /// No error occurred
    None = 0x0000,

    /// Actually, <device(s)> <doesn't/don't> support that functionality.
    FunctionNotSupported = 0x0001,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter, strum::Display)]
#[repr(u8)]
pub enum DeviceStatus {
    /// Confirm that the command succeeded.
    Success,

    /// Target device is unable to perform the command.
    Error,
}

impl std::convert::TryFrom<u8> for DeviceStatus {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, ()> {
        Self::iter().find(|e| e.clone() as u8 == v).ok_or(())
    }
}

impl rand::distributions::Distribution<DeviceStatus> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> DeviceStatus {
        DeviceStatus::iter()
            .nth(rng.gen_range(0..DeviceStatus::iter().len()))
            .unwrap()
    }
}

impl std::convert::TryFrom<u16> for DeviceError {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Self::iter().find(|e| e.clone() as u16 == v).ok_or(())
    }
}

impl rand::distributions::Distribution<DeviceError> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> DeviceError {
        DeviceError::iter()
            .nth(rng.gen_range(0..DeviceError::iter().len()))
            .unwrap()
    }
}

impl std::convert::TryFrom<u16> for DeviceCommand {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Self::iter().find(|e| e.clone() as u16 == v).ok_or(())
    }
}

impl rand::distributions::Distribution<DeviceCommand> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> DeviceCommand {
        DeviceCommand::iter()
            .nth(rng.gen_range(0..DeviceCommand::iter().len()))
            .unwrap()
    }
}

#[cfg(feature = "postgres-types")]
impl<'a> postgres_types::FromSql<'a> for DeviceType {
    fn from_sql(
        _ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        use std::str::FromStr;

        let str = std::str::from_utf8(raw)?;
        Ok(Self::from_str(str)?)
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        *ty == postgres_types::Type::TEXT
    }
}

#[cfg(feature = "postgres-types")]
impl<'a> postgres_types::FromSql<'a> for DeviceTrait {
    fn from_sql(
        _ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        use std::str::FromStr;

        let str = std::str::from_utf8(raw)?;
        Ok(Self::from_str(str)?)
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        *ty == postgres_types::Type::TEXT
    }
}
