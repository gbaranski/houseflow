use serde::{Deserialize, Serialize};

pub mod request {
    use super::*;

    /// QUERY request payload.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        /// List of target devices.
        pub devices: Vec<Device>,
    }

    /// QUERY request payload.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Device {
        /// Device ID, as per the ID provided in SYNC.
        pub id: String,

        /// If the opaque customData object is provided in SYNC, it's sent here.
        pub custom_data: Option<serde_json::Map<String, serde_json::Value>>,
    }
}

pub mod response {
    use super::*;

    /// QUERY response.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub request_id: String,
        pub payload: Payload,
    }

    /// QUERY response payload.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
        pub devices: std::collections::HashMap<String, Device>,
    }

    /// Device execution result.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Device {
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

      /// Result of the query operation.
      #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
      #[repr(u8)]
      #[serde(rename_all = "UPPERCASE")]
      pub enum DeviceStatus {
          /// Confirm that the query succeeded.
          Success,
  
          /// Target device is in offline state or unreachable.
          Offline,
  
          /// There is an issue or alert associated with a query.
          /// The query could succeed or fail.
          /// This status type is typically set when you want to send additional information about another connected device.
          Exceptions,
  
          /// Unable to query the target device.
          Error,
      }
}
