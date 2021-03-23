use houseflow_db::{Database, models::{ User, Device }};
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use crate::Error;

pub mod request {
    use super::*;

    #[derive(Deserialize)]
    pub struct PayloadCommandDevice {
      /// Device ID, as per the ID provided in SYNC.
      pub id: String,
    }

    #[derive(Deserialize)]
    pub struct PayloadCommandExecution {
      /// The command to execute, usually with accompanying parameters.
      pub command: String,

      /// Aligned with the parameters for each command.
      pub params: std::collections::HashMap<String, String>
    }

    #[derive(Deserialize)]
    pub struct PayloadCommand {
      /// List of target devices.
      pub devices: Vec<PayloadCommandDevice>,

      /// List of commands to execute on target devices.
      #[serde(rename = "execution")] // why the fuck this is singular if its vector?
      pub executions: Vec<PayloadCommandExecution>

    }

    #[derive(Deserialize)]
    pub struct Payload {
      /// List of device target and command pairs.
      pub commands: Vec<PayloadCommand>
    }
}

pub mod response {
    use super::*;

    #[derive(Serialize)]
    pub enum PayloadCommandStatus {
        /// Confirm that the command succeeded.
        #[serde(rename = "SUCCESS")]
        Success,

        /// Command is enqueued but expected to succeed.
        #[serde(rename = "PENDING")]
        Pending,
        
        /// Target device is in offline state or unreachable.
        #[serde(rename = "OFFLINE")]
        Offline,
        
        /// There is an issue or alert associated with a command. 
        /// The command could succeed or fail. 
        /// This status type is typically set when you want to send additional information about another connected device.
        #[serde(rename = "EXCEPTIONS")]
        Exceptions,

        /// Target device is unable to perform the command.
        #[serde(rename = "ERROR")]
        Error
    }

    #[derive(Serialize)]
    pub struct PayloadCommand {
        /// List of device IDs corresponding to this status.
        pub ids: Vec<Uuid>,

        /// Result of the execute operation.
        pub status: PayloadCommandStatus,

        /// Aligned with per-trait states described in each trait schema reference. 
        /// These are the states after execution, if available.
        pub states: std::collections::HashMap<String, String>,

        /// Expanding ERROR state if needed from the preset error codes, which will map to the errors presented to users.
        #[serde(rename = "errorCode")]
        pub error_code: Option<String>,
    }


    /// Intent response payload.
    #[derive(Serialize)]
    pub struct Payload {
        /// For systematic errors on SYNC
        #[serde(rename = "errorCode")]
        pub error_code: Option<String>,

        /// Detailed error which will never be presented to users but may be logged or used during development.
        #[serde(rename = "debugString")]
        pub debug_string: Option<String>,

        /// Each object contains one or more devices with response details.
        /// N.B. These may not be grouped the same way as in the request.
        /// For example, the request might turn 7 lights on, with 3 lights succeeding and 4 failing,
        /// thus with two groups in the response.
        pub commands: Vec<Device>,
    }

    #[derive(Serialize)]
    pub struct Response {
        /// ID of the request.
        #[serde(rename = "requestId")]
        pub request_id: String,

        /// Intent response payload.
        pub payload: Payload,
    }

}

use response::Response;

pub async fn handle(
    db: &Database, 
    user: &User, 
    request_payload: request::Payload, 
    request_id: String
) -> Result<Response, Error> {
    log::debug!("Received Execute intent from User ID: {}", user.id.to_string());

    // combined version of all commands and devices as vector of tuple 
    type CombinedDeviceExecution<'a> = Vec<(&'a request::PayloadCommandExecution, &'a request::PayloadCommandDevice)>;

    let requests = request_payload.commands
        .iter()
        .flat_map(|cmd| 
            cmd.executions
                .iter()
                .zip(cmd.devices.iter())
        )
        .collect::<CombinedDeviceExecution>();

    let devices = db.get_user_devices(user.id).await?;


    Err(Error::UserNotFound) // to be changed
    
    // return Ok(Response {
    //     request_id,
    //     payload: response::Payload {
    //         user_id: user.id,
    //         error_code: None,
    //         debug_string: None,
    //         devices,
    //     }
    // })
}
