use crate::device;
use serde::Deserialize;
use serde::Serialize;

/// Request types of the SYNC intent
pub mod request {
    use super::*;

    /// SYNC request payload.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {}
}

/// Response types of the SYNC intent
pub mod response {
    use super::*;

    /// SYNC response
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub request_id: String,
        pub payload: Payload,
    }

    /// SYNC response payload.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        /// Reflects the unique (and immutable) user ID on the agent's platform.
        pub agent_user_id: String,
        /// An error code for the entire transaction for auth failures and developer system unavailability.
        /// For individual device errors, use the errorCode within the device object.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error_code: Option<String>,
        /// Detailed error which will never be presented to users but may be logged or used during development.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub debug_string: Option<String>,
        /// Reflects the unique (and immutable) user ID on the agent's platform.
        pub devices: Vec<PayloadDevice>,
    }

    /// Device execution result.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadDevice {
        /// The ID of the device in the developer's cloud.
        /// This must be unique for the user and for the developer,
        /// as in cases of sharing we may use this to dedupe multiple views of the same device.
        /// It should be immutable for the device; if it changes, the Assistant will treat it as a new device.
        pub id: String,
        /// The hardware type of device.
        #[serde(rename = "type")]
        pub device_type: device::Type,
        /// List of traits this device has. This defines the commands, attributes, and states that the device supports.
        pub traits: Vec<device::Trait>,
        /// Names of this device.
        pub name: PayloadDeviceName,
        /// Indicates whether this device will have its states updated by the Real Time Feed.
        /// true to use the Real Time Feed for reporting state, and false to use the polling model.
        pub will_report_state: bool,
        /// Indicates whether notifications are enabled for the device.
        #[serde(default)]
        pub notification_supported_by_agent: bool,
        /// Provides the current room of the device in the user's home to simplify setup.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub room_hint: Option<String>,
        /// Contains fields describing the device for use in one-off logic if needed (e.g. 'broken firmware version X of light Y requires adjusting color', or 'security flaw requires notifying all users of firmware Z').
        #[serde(skip_serializing_if = "Option::is_none")]
        pub device_info: Option<PayloadDeviceInfo>,
        /// Aligned with per-trait attributes described in each trait schema reference.
        #[serde(default)]
        pub attributes: serde_json::Map<String, serde_json::Value>,
        /// Object defined by the developer which will be attached to future QUERY and EXECUTE requests, maximum of 512 bytes per device. Use this object to store additional information about the device your cloud service may need, such as the global region of the device. Data in this object has a few constraints: No sensitive information, including but not limited to Personally Identifiable Information.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub custom_data: Option<serde_json::Map<String, serde_json::Value>>,
        /// List of alternate IDs used to identify a cloud synced device for local execution.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub other_device_ids: Option<Vec<PayloadOtherDeviceID>>,
    }

    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Attributes {
        // Attributes for ColorSetting trait.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color_model: Option<ColorModel>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color_temperature_range: Option<ColorTemperatureRange>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub command_only_color_setting: Option<bool>,

        // Attributes for TemperatureSetting trait.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub available_thermostat_modes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub buffer_range_celsius: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub command_only_temperature_setting: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub query_only_temperature_setting: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub thermostat_temperature_range: Option<ThermostatTemperatureRange>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub thermostat_temperature_unit: Option<ThermostatTemperatureUnit>,
    }

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ColorTemperatureRange {
        pub temperature_min_k: u64,
        pub temperature_max_k: u64,
    }

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub enum ColorModel {
        Rgb,
        Hsv,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ThermostatTemperatureRange {
        pub min_threshold_celsius: f64,
        pub max_threshold_celcius: f64,
    }

    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    pub enum ThermostatTemperatureUnit {
        C,
        F,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadDeviceName {
        /// List of names provided by the developer rather than the user, often manufacturer names, SKUs, etc.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_names: Option<Vec<String>>,
        /// Primary name of the device, generally provided by the user.
        /// This is also the name the Assistant will prefer to describe the device in responses.
        pub name: String,
        /// Additional names provided by the user for the device.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub nicknames: Option<Vec<String>>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadDeviceInfo {
        /// Especially useful when the developer is a hub for other devices.
        /// Google may provide a standard list of manufacturers here so that e.g. TP-Link and Smartthings both describe 'osram' the same way.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub manufacturer: Option<String>,
        /// The model or SKU identifier of the particular device.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub model: Option<String>,
        /// Specific version number attached to the hardware if available.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hw_version: Option<String>,
        /// Specific version number attached to the software/firmware, if available.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sw_version: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadOtherDeviceID {
        /// The agent's ID. Generally, this is the project ID in the Actions console.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub agent_id: Option<String>,
        /// Device ID defined by the agent. The device ID must be unique.
        pub device_id: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn color_setting_attributes() {
        let attributes = response::Attributes {
            color_model: Some(response::ColorModel::Rgb),
            command_only_color_setting: Some(true),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_string(&attributes).unwrap(),
            json!({"colorModel": "rgb", "commandOnlyColorSetting": true}).to_string()
        );
    }
}
