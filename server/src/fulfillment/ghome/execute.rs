use super::homie::color_absolute_to_property_value;
use super::homie::get_homie_device_by_id;
use super::homie::percentage_to_property_value;
use crate::State;
use google_smart_home::device::commands as ghome_commands;
use google_smart_home::device::Command as GHomeCommand;
use google_smart_home::execute::request;
use google_smart_home::execute::request::PayloadCommandDevice;
use google_smart_home::execute::request::PayloadCommandExecution;
use google_smart_home::execute::response;
use homie_controller::Device;
use homie_controller::HomieController;
use houseflow_types::device;
use houseflow_types::device::commands;
use houseflow_types::errors::InternalError;
use houseflow_types::user;
use std::collections::HashMap;
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

    let commands = if let Some(homie_controller) = state.homie_controllers.get(&user_id) {
        execute_homie_devices(homie_controller, &homie_controller.devices(), requests).await
    } else {
        execute_lighthouse_devices(&state, &user_id, requests).await?
    };

    Ok(response::Payload {
        error_code: None,
        debug_string: None,
        commands,
    })
}

async fn execute_homie_devices<'a>(
    controller: &HomieController,
    devices: &HashMap<String, Device>,
    requests: impl Iterator<Item = (&'a PayloadCommandExecution, &'a PayloadCommandDevice)>,
) -> Vec<response::PayloadCommand> {
    let mut responses = vec![];
    for (execution, device) in requests {
        responses.push(execute_homie_device(controller, devices, execution, device).await);
    }
    responses
}

async fn execute_homie_device(
    controller: &HomieController,
    devices: &HashMap<String, Device>,
    execution: &PayloadCommandExecution,
    command_device: &PayloadCommandDevice,
) -> response::PayloadCommand {
    let ids = vec![command_device.id.to_owned()];

    if let Some((device, node)) = get_homie_device_by_id(devices, &command_device.id) {
        // TODO: Check if device is offline?
        match &execution.command {
            GHomeCommand::OnOff(onoff) => {
                if let Some(on) = node.properties.get("on") {
                    return if controller
                        .set(&device.id, &node.id, "on", onoff.on)
                        .await
                        .is_err()
                    {
                        response::PayloadCommand {
                            ids,
                            status: response::PayloadCommandStatus::Error,
                            states: Default::default(),
                            error_code: Some("transientError".to_string()),
                        }
                    } else {
                        response::PayloadCommand {
                            ids,
                            status: response::PayloadCommandStatus::Pending,
                            states: Default::default(),
                            error_code: None,
                        }
                    };
                }
            }
            GHomeCommand::BrightnessAbsolute(brightness_absolute) => {
                if let Some(brightness) = node.properties.get("brightness") {
                    if let Some(value) =
                        percentage_to_property_value(brightness, brightness_absolute.brightness)
                    {
                        return if controller
                            .set(&device.id, &node.id, "brightness", value)
                            .await
                            .is_err()
                        {
                            response::PayloadCommand {
                                ids,
                                status: response::PayloadCommandStatus::Error,
                                states: Default::default(),
                                error_code: Some("transientError".to_string()),
                            }
                        } else {
                            response::PayloadCommand {
                                ids,
                                status: response::PayloadCommandStatus::Pending,
                                states: Default::default(),
                                error_code: None,
                            }
                        };
                    }
                }
            }
            GHomeCommand::ColorAbsolute(color_absolute) => {
                if let Some(color) = node.properties.get("color") {
                    if let Some(value) = color_absolute_to_property_value(color, color_absolute) {
                        return if controller
                            .set(&device.id, &node.id, "color", value)
                            .await
                            .is_err()
                        {
                            response::PayloadCommand {
                                ids,
                                status: response::PayloadCommandStatus::Error,
                                states: Default::default(),
                                error_code: Some("transientError".to_string()),
                            }
                        } else {
                            response::PayloadCommand {
                                ids,
                                status: response::PayloadCommandStatus::Pending,
                                states: Default::default(),
                                error_code: None,
                            }
                        };
                    }
                }
            }
            _ => {}
        }
        response::PayloadCommand {
            ids,
            status: response::PayloadCommandStatus::Error,
            states: Default::default(),
            error_code: Some("actionNotAvailable".to_string()),
        }
    } else {
        response::PayloadCommand {
            ids,
            status: response::PayloadCommandStatus::Error,
            states: Default::default(),
            error_code: Some("deviceNotFound".to_string()),
        }
    }
}

async fn execute_lighthouse_devices(
    state: &State,
    user_id: &user::ID,
    requests: impl Iterator<Item = (&PayloadCommandExecution, &PayloadCommandDevice)>,
) -> Result<Vec<response::PayloadCommand>, InternalError> {
    let sessions = &state.sessions;
    let config = &state.config;

    let responses = requests.map(|(execution, device)| async move {
        let device_id = device::ID::from_str(&device.id).expect("invalid device ID");
        let ids = [device.id.clone()].to_vec();
        if config.get_permission(&device_id, user_id).is_none() {
            return Ok::<_, InternalError>(response::PayloadCommand {
                ids,
                status: response::PayloadCommandStatus::Error,
                states: Default::default(),
                error_code: Some(String::from("authFailure")),
            });
        }
        let session = match sessions.get(&device_id) {
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
                device::Command::OnOff(commands::OnOff { on })
            }
            GHomeCommand::OpenClose(ghome_commands::OpenClose { open_percent }) => {
                device::Command::OpenClose(commands::OpenClose { open_percent })
            }
            _ => todo!(),
        };

        let request = houseflow_types::lighthouse::execute::Frame {
            id: rand::random(),
            command: command.clone(),
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
            device::Status::Success => response::PayloadCommand {
                ids,
                status: response::PayloadCommandStatus::Success,
                states: response.state,
                error_code: None,
            },
            device::Status::Error(error) => response::PayloadCommand {
                ids,
                status: response::PayloadCommandStatus::Error,
                states: response.state,
                error_code: Some(error.to_string()),
            },
        })
    });
    futures::future::try_join_all(responses).await
}
