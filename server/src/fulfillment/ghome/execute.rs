use crate::State;
use google_smart_home::device::commands as ghome_commands;
use google_smart_home::device::Command as GHomeCommand;
use google_smart_home::execute::request;
use google_smart_home::execute::response;
use houseflow_types::accessory;
use houseflow_types::accessory::commands;
use houseflow_types::errors::InternalError;
use houseflow_types::structure;
use houseflow_types::user;
use std::str::FromStr;

#[tracing::instrument(name = "Execute", skip(state), err)]
pub async fn handle(
    state: State,
    user_id: user::ID,
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
        let accessory_id = accessory::ID::from_str(&device.id).expect("invalid accessory ID");
        let ids = [device.id.clone()].to_vec();
        let structure_id = structure::ID::parse_str(
            &device
                .custom_data
                .get("structure-id")
                .unwrap()
                .as_str()
                .unwrap(),
        )
        .unwrap();
        if config.get_permission(&structure_id, user_id).is_none() {
            return Ok::<_, InternalError>(response::PayloadCommand {
                ids,
                status: response::PayloadCommandStatus::Error,
                states: Default::default(),
                error_code: Some(String::from("authFailure")),
            });
        }
        let session = match sessions.get(&accessory_id) {
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

        let command = match execution.command {
            GHomeCommand::OnOff(ghome_commands::OnOff { on }) => {
                accessory::Command::OnOff(commands::OnOff { on })
            }
            GHomeCommand::OpenClose(ghome_commands::OpenClose { open_percent }) => {
                accessory::Command::OpenClose(commands::OpenClose { open_percent })
            }
            _ => todo!(),
        };

        let request = houseflow_types::lighthouse::AccessoryExecuteFrame {
            id: rand::random(),
            command: command.clone(),
        };
        let response = match tokio::time::timeout(
            crate::fulfillment::EXECUTE_TIMEOUT,
            session.accessory_execute(request),
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

        tracing::info!(command = ?command, status = %response.status, "Executed command on device");

        Ok(match response.status {
            accessory::Status::Success => response::PayloadCommand {
                ids,
                status: response::PayloadCommandStatus::Success,
                states: response.state,
                error_code: None,
            },
            accessory::Status::Error(error) => response::PayloadCommand {
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
