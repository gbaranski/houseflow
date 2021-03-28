use houseflow_db::models::User;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::intent::IntentError as Error;

pub mod request {
    use super::*;

    #[derive(Deserialize)]
    pub struct PayloadCommandDevice {
      /// Device ID, as per the ID provided in SYNC.
      pub id: Uuid,
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
    pub struct PayloadCommand {
        /// List of device IDs corresponding to this status.
        pub ids: Vec<Uuid>,

        /// Result of the execute operation.
        pub status: houseflow_lighthouse::PayloadCommandStatus,

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
        pub commands: Vec<PayloadCommand>,
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
use houseflow_lighthouse::{
    Response as LighthouseResponse,
    Error as LighthouseError,
};

pub async fn handle<'a>(
    app_state: &crate::AppState<'a>,
    user: &User, 
    request_payload: request::Payload, 
    request_id: String
) -> Result<Response, Error> {
    log::debug!("Received Execute intent from User ID: {}", user.id.to_string());

    let requests = request_payload.commands
        .iter()
        .flat_map(|cmd| 
            cmd.executions
                .iter()
                .zip(cmd.devices.iter())
        );

    type CombinedLighthouseResponse = (Uuid, Result<LighthouseResponse, Error>);
    let responses: Vec<CombinedLighthouseResponse> = join_all(requests
        .map(|(exec, device)| async move {
            let get_resp_fn = || async move {
                let is_allowed = app_state.db.get_device_permission(user.id, device.id)
                    .await?
                    .map_or(
                        Err(Error::NoDevicePermission), 
                        |v| if v.execute { Err(Error::NoDeviceExecutePermission) } else {Ok(())}
                    )?;

                let addr = app_state.lighthouse.get_wealthy_lighthouse_address(&device.id)?
                    .ok_or(Error::NoWealthyLighthouse)?;
                
                let resp = app_state.lighthouse.send_execute(addr, houseflow_lighthouse::ExecuteRequest {
                    params: exec.params.clone(),
                    command: exec.command.clone(),
                    device_id: device.id,
                }).await?;

                Ok::<LighthouseResponse, Error>(resp)
            };

            (device.id, get_resp_fn().await)
        })).await;


    Ok(
        Response {
            request_id,
            payload: response::Payload {
                error_code: None,
                debug_string: None,
                commands: responses
                    .iter()
                    .map(|(device_id, resp)| response::PayloadCommand {
                        ids: vec![*device_id],
                        states: resp.map_or_else(|_| std::collections::HashMap::new(), |v| v.states),
                        error_code: resp.map_or_else(|e| Some(e.to_string()), |v| v.error_code),
                        status: resp.map_or(houseflow_lighthouse::ResponseStatus::Error, |v| v.status),
                    })
                    .collect()
            }
        }
    )
}
