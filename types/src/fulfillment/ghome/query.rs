use super::DeviceStatus;
use crate::DeviceID;
use serde::{Deserialize, Serialize};

pub mod request {
    use super::*;

    /// QUERY request payload.
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        /// List of target devices.
        pub devices: Vec<PayloadDevice>,
    }

    /// QUERY request payload.
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadDevice {
        /// Device ID, as per the ID provided in SYNC.
        pub id: DeviceID,

        /// If the opaque customData object is provided in SYNC, it's sent here.
        pub custom_data: Option<serde_json::Map<String, serde_json::Value>>,
    }
}

pub mod response {
    use super::*;

    /// SYNC response payload.
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        /// An error code for the entire transaction for auth failures and developer system unavailability.
        /// For individual device errors use the errorCode within the device object.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error_code: Option<String>,

        /// Detailed error which will never be presented to users but may be logged or used during development.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub debug_string: Option<String>,

        /// Map of devices. Maps developer device ID to object of state properties.
        pub devices: std::collections::HashMap<DeviceID, PayloadDevice>,
    }

    /// Device execution result.
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadDevice {
        /// Indicates if the device is online (that is, reachable) or not.
        pub online: bool,

        /// Result of the query operation.
        pub status: DeviceStatus,

        /// Expanding ERROR state if needed from the preset error codes, which will map to the errors presented to users.
        pub error_code: Option<String>,

        /// Device state
        #[serde(flatten)]
        pub state: Option<serde_json::Map<String, serde_json::Value>>,
    }
}
