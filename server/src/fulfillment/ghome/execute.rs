use crate::State;
use google_smart_home::execute::{request, response};
use houseflow_types::{errors::InternalError, DeviceCommand, DeviceID, UserID};
use std::str::FromStr;

#[tracing::instrument(name = "Execute", skip(state), err)]
pub async fn handle(
    state: State,
    user_id: UserID,
    payload: &request::Payload,
) -> Result<response::Payload, InternalError> {
    let requests = payload
        .commands
        .iter()
        .flat_map(|cmd| cmd.execution.iter().zip(cmd.devices.iter()));

    let sessions = &state.sessions;
    let config = &state.config;
    let user_id = &user_id;

    let responses = requests.map(|(execution, device)| async move {
        let device_id = DeviceID::from_str(&device.id).expect("invalid device ID");
        let ids = [device.id.clone()].to_vec();
        if config.get_permission(&device_id, user_id).is_none() {
            return Ok::<_, InternalError>(response::PayloadCommand {
                ids,
                status: response::PayloadCommandStatus::Error,
                states: Default::default(),
                error_code: Some(String::from("authFailure")),
            });
        }
        let session = match sessions.lock().unwrap().get(&device_id) {
            Some(session) => session.clone(),
            None => {
                return Ok(response::PayloadCommand {
                    ids,
                    status: response::PayloadCommandStatus::Offline,
                    states: Default::default(),
                    error_code: Some(String::from("offline")),
                })
            }
        };

        let command = DeviceCommand::from_str(
            &execution
                .command
                .strip_prefix(format!("{}.", super::COMMAND_PREFIX).as_str())
                .unwrap(),
        )
        .expect("invalid command");
        let request = houseflow_types::lighthouse::proto::execute::Frame {
            id: rand::random(),
            command: command.clone(),
            params: execution.params.clone(),
        };
        let response = match tokio::time::timeout(
            crate::fulfillment::EXECUTE_TIMEOUT,
            session.execute(request),
        )
        .await
        {
            Ok(val) => val?,
            Err(_) => {
                return Ok(response::PayloadCommand {
                    ids,
                    status: response::PayloadCommandStatus::Offline,
                    states: Default::default(),
                    error_code: Some(String::from("offline")),
                })
            }
        };

        tracing::info!(command = %command, status = %response.status, "Executed command on device");

        Ok(match response.status {
            houseflow_types::DeviceStatus::Success => response::PayloadCommand {
                ids,
                status: response::PayloadCommandStatus::Success,
                states: response.state,
                error_code: None,
            },
            houseflow_types::DeviceStatus::Error(error) => response::PayloadCommand {
                ids,
                status: response::PayloadCommandStatus::Error,
                states: response.state,
                error_code: Some(error.to_string()),
            },
        })
    });

    Ok(response::Payload {
        error_code: None,
        debug_string: None,
        commands: futures::future::try_join_all(responses).await?,
    })
}
