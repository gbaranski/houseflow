use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub const DEVICE_SCHEMA: &str = r#"
CREATE EXTENSION IF NOT EXISTS hstore;
CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL,
    type TEXT NOT NULL,
    traits TEXT[] NOT NULL,
    
    default_names TEXT[] NOT NULL,
    name TEXT NOT NULL,
    nicknames TEXT[] NOT NULL,

    will_report_state BOOL NOT NULL,
    notification_support_by_agent BOOL NOT NULL,

    room_hint TEXT NOT NULL,

    manufacturer TEXT NOT NULL DEFAULT,
    model TEXT NOT NULL,
    hw_version TEXT NOT NULL,
    sw_version TEXT NOT NULL,

    attributes hstore NOT NULL,

    pkey_base64 CHAR(44) NOT NULL,

    PRIMARY KEY (id)
);
    
"#;


/// Contains fields describing the device for use in one-off logic if needed.
/// e.g. 'broken firmware version X of light Y requires adjusting color', or 'security flaw requires notifying all users of firmware Z'.
#[derive(Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Especially useful when the developer is a hub for other devices. 
    /// Google may provide a standard list of manufacturers here so that e.g. TP-Link and Smartthings both describe 'osram' the same way.
    pub manufacturer: String,

    /// The model or SKU identifier of the particular device.
    pub model: String,

    /// Specific version number attached to the hardware if available.
    #[serde(rename = "hwVersion")]
    pub hw_version: String,

    /// Specific version number attached to the software/firmware, if available.
    #[serde(rename = "swVersion")]
    pub sw_version: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceName {
    /// List of names provided by the developer rather than the user, often manufacturer names, SKUs, etc.
    pub default_names: Vec<String>,

    /// Primary name of the device, generally provided by the user. This is also the name the Assistant will prefer to describe the device in responses.
    pub name: String,

    /// Additional names provided by the user for the device.
    pub nicknames: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    /// The ID of the device in the developer's cloud. 
    /// This must be unique for the user and for the developer, 
    /// as in cases of sharing we may use this to dedupe multiple views of the same device. 
    /// It should be immutable for the device; if it changes, the Assistant will treat it as a new device.
    pub id: String,

    /// The hardware type of device.
    #[serde(rename = "type")]
    pub device_type: String,

    /// List of traits this device has. 
    /// This defines the commands, attributes, and states that the device supports.
    pub traits: Vec<String>,

    /// Names of this device.
    pub name: DeviceName,

    /// Indicates whether this device will have its states updated by the Real Time Feed.
    /// true to use the Real Time Feed for reporting state, and false to use the polling model.
    #[serde(rename = "willReportState")]
    pub will_report_state: bool,

    /// Indicates whether notifications are enabled for the device.
    #[serde(rename = "notificationSupportedByAgent", default)]
    pub notification_support_by_agent: bool,

    /// Provides the current room of the device in the user's home to simplify setup.
    #[serde(rename = "roomHint")]
    pub room_hint: String,

    /// Contains fields describing the device for use in one-off logic if needed 
    /// e.g. 'broken firmware version X of light Y requires adjusting color', or 'security flaw requires notifying all users of firmware Z'.
    #[serde(rename = "deviceInfo")]
    pub device_info: DeviceInfo,

    /// Aligned with per-trait attributes described in each trait schema reference.
    pub attributes: HashMap<String, Option<String>>,

    /// ED25519 Public key, Base64 encoded, 44 in size
    #[serde(skip)]
    pub pkey_base64: String,
}


impl Device {
    pub async fn by_id(db: &crate::Database, id: String) -> Result<Option<Device>, crate::Error> {
        const SQL_QUERY: &str = 
        r#"
        "SELECT 
            type, 
            traits, 
            default_names, 
            name, 
            nicknames, 
            will_report_state, 
            notification_support_by_agent, 
            room_hint, 
            manufacturer, 
            model, 
            hw_version, 
            sw_version, 
            attributes, 
            pkey_base64 
        FROM 
            devices 
        WHERE 
            id=$1"
        "#;
        let row = db.client
            .query_one(SQL_QUERY, &[&id])
            .await?;

        if row.is_empty() {
            return Ok(None)
        }

        Ok(Some(Device{
            id,
            device_type: row.try_get(0)?,
            traits: row.try_get(1)?,
            name: DeviceName{
                default_names: row.try_get(2)?,
                name: row.try_get(3)?,
                nicknames: row.try_get(4)?,
            },
            will_report_state: row.try_get(5)?,
            notification_support_by_agent: row.try_get(6)?,
            room_hint: row.try_get(7)?,
            device_info: DeviceInfo{
                manufacturer: row.try_get(8)?,
                model: row.try_get(9)?,
                hw_version: row.try_get(10)?,
                sw_version: row.try_get(11)?,
            },
            attributes: row.try_get(12)?,
            pkey_base64: row.try_get(13)?,
        }))
    }
}
