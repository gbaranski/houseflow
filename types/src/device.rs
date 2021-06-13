use crate::common::Credential;
use semver::Version;
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type DeviceID = Credential<16>;
pub type DevicePassword = Credential<32>;

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

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DevicePermission {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}


use strum_macros::EnumString;

/// Traits defines what functionality device supports
#[derive(Debug, Clone, Hash, Eq, PartialEq, strum_macros::Display, EnumString)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeviceTrait {}

/// Type of the device
#[derive(Debug, Clone, PartialEq, Eq, strum_macros::Display, EnumString)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeviceType {
    Gate
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
        *ty == postgres_types::Type::TEXT_ARRAY
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
        *ty == postgres_types::Type::TEXT_ARRAY
    }
}
