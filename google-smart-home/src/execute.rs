use crate::device::Command;
use serde::Deserialize;
use serde::Serialize;

/// Request types of the EXECUTE intent
pub mod request {
    use super::*;

    /// EXECUTE request payload.
    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        /// List of device target and command pairs.
        pub commands: Vec<PayloadCommand>,
    }

    /// Set of commands to execute on the attached device targets.
    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadCommand {
        /// List of target devices.
        pub devices: Vec<PayloadCommandDevice>,
        /// List of commands to execute on target devices.
        pub execution: Vec<PayloadCommandExecution>,
    }

    /// Device target to execute.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadCommandDevice {
        /// Device ID, as per the ID provided in SYNC.
        pub id: String,
        /// If the opaque customData object is provided in SYNC, it's sent here.
        #[serde(default)]
        pub custom_data: serde_json::Map<String, serde_json::Value>,
    }

    /// Device command.
    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadCommandExecution {
        /// The command to execute, usually with accompanying parameters.
        #[serde(flatten)]
        pub command: Command,
    }
}

/// Response types of the EXECUTE intent
pub mod response {
    use super::*;

    /// EXECUTE response.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub request_id: String,
        pub payload: Payload,
    }

    /// EXECUTE response payload.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        /// An error code for the entire transaction for auth failures and developer system unavailability.
        /// For individual device errors, use the errorCode within the device object.
        pub error_code: Option<String>,
        /// Detailed error which will never be presented to users but may be logged or used during development.
        pub debug_string: Option<String>,
        /// Each object contains one or more devices with response details. N.B.
        /// These may not be grouped the same way as in the request.
        /// For example, the request might turn 7 lights on, with 3 lights succeeding and 4 failing, thus with two groups in the response
        pub commands: Vec<PayloadCommand>,
    }

    /// Device execution result.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadCommand {
        /// List of device IDs corresponding to this status.
        pub ids: Vec<String>,
        /// Result of the execute operation.
        pub status: PayloadCommandStatus,
        /// Aligned with per-trait states described in each trait schema reference.
        /// These are the states after execution, if available.
        #[serde(default)]
        #[serde(skip_serializing_if = "serde_json::Map::is_empty")]
        pub states: serde_json::Map<String, serde_json::Value>,
        /// Expanding ERROR state if needed from the preset error codes, which will map to the errors presented to users.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error_code: Option<String>,
    }

    /// Result of the execute operation.
    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    #[repr(u8)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum PayloadCommandStatus {
        /// Confirm that the command succeeded.
        Success,
        /// Command is enqueued but expected to succeed.
        Pending,
        /// Target device is in offline state or unreachable.
        Offline,
        /// There is an issue or alert associated with a command.
        /// The command could succeed or fail.
        /// This status type is typically set when you want to send additional information about another connected device.
        Exceptions,
        /// Target device is unable to perform the command.
        Error,
    }
}
