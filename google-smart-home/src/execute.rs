use serde::{Deserialize, Serialize};

/// Request types of the EXECUTE intent
pub mod request {
    use super::*;

    /// EXECUTE request payload.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        /// List of device target and command pairs.
        pub commands: Vec<PayloadCommand>,
    }

    /// Set of commands to execute on the attached device targets.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
        pub custom_data: Option<serde_json::Map<String, serde_json::Value>>,
    }

    /// Device command.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PayloadCommandExecution {
        /// The command to execute, usually with accompanying parameters.
        pub command: String,

        /// Aligned with the parameters for each command.
        #[serde(default)]
        pub params: serde_json::Map<String, serde_json::Value>,
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
        pub commands: Vec<Command>,
    }

    /// Device execution result.
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Command {
        /// List of device IDs corresponding to this status.
        pub ids: Vec<String>,

        /// Result of the execute operation.
        pub status: CommandStatus,

        /// Aligned with per-trait states described in each trait schema reference.
        /// These are the states after execution, if available.
        #[serde(default)]
        pub states: serde_json::Map<String, serde_json::Value>,

        /// Expanding ERROR state if needed from the preset error codes, which will map to the errors presented to users.
        pub error_code: Option<String>,
    }

    /// Result of the execute operation.
    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    #[repr(u8)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum CommandStatus {
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

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    mod request {
        use super::{
            super::request::{
                Payload, PayloadCommand, PayloadCommandDevice, PayloadCommandExecution,
            },
            lazy_static,
        };

        use crate::{Request, RequestInput};
        use serde_json::json;

        const JSON: &str = include_str!("../samples/execute/request.json");
        lazy_static! {
            static ref EXPECTED: Request = Request {
                request_id: String::from("ff36a3cc-ec34-11e6-b1a0-64510650abcf"),
                inputs: vec![RequestInput::Execute(Payload {
                    commands: vec![PayloadCommand {
                        devices: vec![
                            PayloadCommandDevice {
                                id: String::from("123"),
                                custom_data: Some(
                                    json!({
                                        "fooValue": 74,
                                        "barValue": true,
                                        "bazValue": "sheepdip"
                                    })
                                    .as_object()
                                    .unwrap()
                                    .clone(),
                                ),
                            },
                            PayloadCommandDevice {
                                id: String::from("456"),
                                custom_data: Some(
                                    json!({
                                        "fooValue": 36,
                                        "barValue": false,
                                        "bazValue": "moarsheep"
                                    })
                                    .as_object()
                                    .unwrap()
                                    .clone(),
                                ),
                            },
                        ],
                        execution: vec![PayloadCommandExecution {
                            command: String::from("action.devices.commands.OnOff"),
                            params: json!({
                                "on": true,
                            })
                            .as_object()
                            .unwrap()
                            .clone(),
                        }],
                    }],
                })]
            };
        }

        #[test]
        fn valid() {
            let parsed = serde_json::from_str::<Request>(JSON).unwrap();
            assert_eq!(parsed, *EXPECTED);
            let json = serde_json::to_string(&*EXPECTED).unwrap();
            let parsed = serde_json::from_str::<Request>(&json).unwrap();
            assert_eq!(parsed, *EXPECTED);
        }
    }

    mod response {
        use super::{
            super::response::{Command, CommandStatus, Payload, Response},
            lazy_static,
        };
        use serde_json::json;

        const JSON: &str = include_str!("../samples/execute/response.json");
        lazy_static! {
            static ref EXPECTED: Response = Response {
                request_id: String::from("ff36a3cc-ec34-11e6-b1a0-64510650abcf"),
                payload: Payload {
                    error_code: None,
                    debug_string: None,
                    commands: vec![
                        Command {
                            ids: vec![String::from("123")],
                            status: CommandStatus::Success,
                            error_code: None,
                            states: json!({
                                "on": true,
                                "online": true,
                            })
                            .as_object()
                            .unwrap()
                            .clone()
                        },
                        Command {
                            ids: vec![String::from("456")],
                            status: CommandStatus::Error,
                            error_code: Some(String::from("deviceTurnedOff")),
                            states: Default::default(),
                        }
                    ]
                }
            };
        }

        #[test]
        fn valid() {
            let parsed = serde_json::from_str::<Response>(JSON).unwrap();
            assert_eq!(parsed, *EXPECTED);
            let json = serde_json::to_string(&*EXPECTED).unwrap();
            let parsed = serde_json::from_str::<Response>(&json).unwrap();
            assert_eq!(parsed, *EXPECTED);
        }
    }
}
