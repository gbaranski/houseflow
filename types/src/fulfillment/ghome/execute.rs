use super::DeviceStatus;
use crate::{DeviceCommand, DeviceID};
use serde::{Deserialize, Serialize};

pub mod request {
    use super::*;

    /// EXECUTE request payload.
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        /// List of device target and command pairs.
        pub commands: Vec<PayloadCommand>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadCommand {
        /// List of target devices.
        pub devices: Vec<PayloadCommandDevice>,

        /// List of commands to execute on target devices.
        pub execution: Vec<PayloadCommandExecution>,
    }

    /// Target device
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadCommandDevice {
        /// Device ID, as per the ID provided in SYNC.
        pub id: DeviceID,

        /// If the opaque customData object is provided in SYNC, it's sent here.
        #[serde(default)]
        pub custom_data: Option<serde_json::Map<String, serde_json::Value>>,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadCommandExecution {
        /// The command to execute, usually with accompanying parameters.
        #[serde(with = "insert_command_prefix")]
        pub command: DeviceCommand,

        /// Aligned with the parameters for each command.
        #[serde(default)]
        pub params: serde_json::Map<String, serde_json::Value>,
    }

    mod insert_command_prefix {
        use super::*;

        const PREFIX: &str = "action.devices.commands.";

        use serde::{
            de::{self, Visitor},
            Deserializer, Serializer,
        };

        use std::fmt;
        pub fn serialize<S>(command: &DeviceCommand, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let concatenated = PREFIX.to_owned() + command.to_string().as_str();
            serializer.serialize_str(concatenated.as_str())
        }

        struct DeviceCommandVisitor;

        impl<'de> Visitor<'de> for DeviceCommandVisitor {
            type Value = DeviceCommand;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(&format!("valid device command with `{}` prefix", PREFIX))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let without_prefix = v.trim_start_matches(PREFIX);
                use std::str::FromStr;
                DeviceCommand::from_str(without_prefix).map_err(|err| match err {
                    strum::ParseError::VariantNotFound => E::custom("variant not found"),
                })
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<DeviceCommand, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(DeviceCommandVisitor)
        }
    }
}

pub mod response {
    use super::*;

    /// EXECUTE response payload.
    #[derive(Debug, Clone, Deserialize, Serialize)]
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
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadCommand {
        /// List of device IDs corresponding to this status.
        pub ids: Vec<DeviceID>,

        /// Result of the execute operation.
        pub status: DeviceStatus,

        /// Aligned with per-trait states described in each trait schema reference.
        /// These are the states after execution, if available.
        pub states: serde_json::Map<String, serde_json::Value>,

        /// Expanding ERROR state if needed from the preset error codes, which will map to the errors presented to users.
        pub error_code: Option<String>,
    }
}
