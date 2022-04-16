use std::str::FromStr;

use crate::extensions::Config;
use crate::providers;
use crate::providers::ProviderExt;
use crate::utils;
use futures::future::join_all;
use google_smart_home::device;
use google_smart_home::execute;
use google_smart_home::query;
use google_smart_home::sync::response;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::manufacturers;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::errors::ServerError;
use houseflow_types::user;

use crate::extractors::UserID;
use axum::extract::Extension;
use axum::Json;
use google_smart_home::Request;
use google_smart_home::RequestInput;
use google_smart_home::Response;

pub async fn handle(
    Extension(master_provider): Extension<providers::MasterHandle>,
    config: Config,
    UserID(user_id): UserID,
    Json(request): Json<Request>,
) -> Result<Json<Response>, ServerError> {
    let input = request.inputs.first().unwrap();

    let body: Response = match input {
        RequestInput::Sync => Response::Sync(google_smart_home::sync::response::Response {
            request_id: request.request_id,
            payload: sync(master_provider, config, user_id).await?,
        }),
        RequestInput::Query(payload) => {
            Response::Query(google_smart_home::query::response::Response {
                request_id: request.request_id,
                payload: query(master_provider, config, user_id, payload).await?,
            })
        }
        RequestInput::Execute(payload) => {
            Response::Execute(google_smart_home::execute::response::Response {
                request_id: request.request_id,
                payload: execute(master_provider, user_id, payload).await?,
            })
        }
        RequestInput::Disconnect => todo!(),
    };

    Ok(Json(body))
}

pub async fn sync(
    master_provider: providers::MasterHandle,
    config: Config,
    user_id: user::ID,
) -> Result<response::Payload, ServerError> {
    let accessories = utils::get_user_accessories(&config, &master_provider, &user_id).await;
    let devices = accessories
        .iter()
        .map(|accessory| {
            let (r#type, traits, attributes) = match &accessory.r#type {
                accessory::Type::XiaomiMijia(accessory_type) => match accessory_type {
                    manufacturers::XiaomiMijia::HygroThermometer => (
                        device::Type::Thermostat,
                        vec![device::Trait::TemperatureControl],
                        response::Attributes {
                            query_only_temperature_setting: Some(true),
                            ..Default::default()
                        },
                    ),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            };

            response::PayloadDevice {
                id: accessory.id.to_string(),
                device_type: r#type,
                traits,
                name: response::PayloadDeviceName {
                    default_names: None,
                    name: accessory.name.clone(),
                    nicknames: None,
                },
                will_report_state: false,
                notification_supported_by_agent: false, // not sure about that
                room_hint: Some(accessory.room_name.clone()),
                device_info: Some(response::PayloadDeviceInfo {
                    manufacturer: Some("houseflow".to_string()),
                    model: None,
                    hw_version: None,
                    sw_version: None,
                }),
                attributes,
                custom_data: None,
                other_device_ids: None,
            }
        })
        .collect::<Vec<_>>();

    Ok(response::Payload {
        agent_user_id: user_id.to_string(),
        error_code: None,
        debug_string: None,
        devices,
    })
}

async fn query(
    master_provider: providers::MasterHandle,
    config: Config,
    user_id: user::ID,
    payload: &query::request::Payload,
) -> Result<query::response::Payload, ServerError> {
    let responses = payload.devices.iter().map(|device| {
        let config = config.clone();
        let master_provider = master_provider.clone();
        async move {
            let accessory_id = accessory::ID::from_str(&device.id).unwrap();
            let permission =
                utils::get_permission(&config, &master_provider, &user_id, &accessory_id).await;
            if permission.is_none() {
                return Ok::<_, ServerError>((
                    device.id.clone(),
                    query::response::PayloadDevice {
                        status: query::response::PayloadDeviceStatus::Error,
                        state: Default::default(),
                        error_code: Some(String::from("authFailure")),
                    },
                ));
            }
            let accessories = master_provider.get_accessories().await;
            let accessory = accessories
                .iter()
                .find_map(|(_, accessories)| {
                    accessories
                        .iter()
                        .find(|accessory| accessory.id == accessory_id)
                })
                .unwrap();
            let values = match accessory.r#type {
                accessory::Type::XiaomiMijia(manufacturers::XiaomiMijia::HygroThermometer) => &[
                    (
                        ServiceName::TemperatureSensor,
                        CharacteristicName::CurrentTemperature,
                    ),
                    (
                        ServiceName::HumiditySensor,
                        CharacteristicName::CurrentHumidity,
                    ),
                ],
                accessory::Type::Houseflow(_) => todo!(),
                _ => todo!(),
            };
            let futures = values
                .into_iter()
                .map(|(service_name, characteristic_name)| {
                    master_provider.read_characteristic(
                        accessory_id,
                        service_name.to_owned(),
                        characteristic_name.to_owned(),
                    )
                });
            let characteristics = join_all(futures).await;
            let mut state = query::response::State {
                online: true,
                ..Default::default()
            };

            for characteristic in characteristics {
                match characteristic {
                    Ok(Characteristic::CurrentTemperature(
                        characteristics::CurrentTemperature { temperature },
                    )) => {
                        state.thermostat_temperature_ambient = Some(temperature as f64);
                    }
                    Ok(Characteristic::CurrentHumidity(characteristics::CurrentHumidity {
                        humidity,
                    })) => {
                        // state.thermostat_humidity_ambient = Some(humidity as f64);
                    }
                    Err(err) => {
                        return Ok((
                            device.id.clone(),
                            query::response::PayloadDevice {
                                status: query::response::PayloadDeviceStatus::Error,
                                error_code: Some(err.to_string()),
                                state,
                            },
                        ))
                    }
                    _ => todo!(),
                };
            }

            Ok((
                device.id.clone(),
                query::response::PayloadDevice {
                    status: query::response::PayloadDeviceStatus::Success,
                    error_code: None,
                    state,
                },
            ))
        }
    });

    Ok(query::response::Payload {
        error_code: None,
        debug_string: None,
        devices: futures::future::try_join_all(responses)
            .await?
            .into_iter()
            .collect(),
    })
}

pub async fn execute(
    master_provider: providers::MasterHandle,
    user_id: user::ID,
    payload: &execute::request::Payload,
) -> Result<execute::response::Payload, ServerError> {
    todo!()
    // let requests = payload
    //     .commands
    //     .iter()
    //     .flat_map(|cmd| cmd.execution.iter().zip(cmd.devices.iter()));

    // let sessions = &state.sessions;
    // let config = &state.config;
    // let user_id = &user_id;

    // let responses = requests.map(|(execution, device)| async move {
    //     let accessory_id = accessory::ID::from_str(&device.id).expect("invalid accessory ID");
    //     let ids = [device.id.clone()].to_vec();
    //     let structure_id = structure::ID::parse_str(
    //         &device
    //             .custom_data
    //             .get("structure-id")
    //             .unwrap()
    //             .as_str()
    //             .unwrap(),
    //     )
    //     .unwrap();
    //     if config.get_permission(&structure_id, user_id).is_none() {
    //         return Ok::<_, InternalError>(response::PayloadCommand {
    //             ids,
    //             status: response::PayloadCommandStatus::Error,
    //             states: Default::default(),
    //             error_code: Some(String::from("authFailure")),
    //         });
    //     }
    //     let session = match sessions.get(&accessory_id) {
    //         Some(session) => session.clone(),
    //         None => {
    //             return Ok(response::PayloadCommand {
    //                 ids,
    //                 status: response::PayloadCommandStatus::Offline,
    //                 states: Default::default(),
    //                 error_code: Some(String::from("offline")),
    //             })
    //         }
    //     };

    //     let command = match execution.command {
    //         GHomeCommand::OnOff(ghome_commands::OnOff { on }) => {
    //             accessory::Command::OnOff(commands::OnOff { on })
    //         }
    //         GHomeCommand::OpenClose(ghome_commands::OpenClose { open_percent }) => {
    //             accessory::Command::OpenClose(commands::OpenClose { open_percent })
    //         }
    //         _ => todo!(),
    //     };

    //     let request = houseflow_types::lighthouse::AccessoryExecuteFrame {
    //         id: rand::random(),
    //         command: command.clone(),
    //     };
    //     let response = match tokio::time::timeout(
    //         crate::fulfillment::EXECUTE_TIMEOUT,
    //         session.accessory_execute(request),
    //     )
    //     .await
    //     {
    //         Ok(val) => val?,
    //         Err(_) => {
    //             return Ok(response::PayloadCommand {
    //                 ids,
    //                 status: response::PayloadCommandStatus::Offline,
    //                 states: Default::default(),
    //                 error_code: Some(String::from("offline")),
    //             })
    //         }
    //     };

    //     tracing::info!(command = ?command, status = %response.status, "Executed command on device");

    //     Ok(match response.status {
    //         accessory::Status::Success => response::PayloadCommand {
    //             ids,
    //             status: response::PayloadCommandStatus::Success,
    //             states: serde_json::json!({}).as_object().unwrap().clone(), // TODO: implement states
    //             error_code: None,
    //         },
    //         accessory::Status::Error(error) => response::PayloadCommand {
    //             ids,
    //             status: response::PayloadCommandStatus::Error,
    //             states: serde_json::json!({}).as_object().unwrap().clone(), // TODO: implement states
    //             error_code: Some(error.to_string()),
    //         },
    //     })
    // });

    // Ok(response::Payload {
    //     error_code: None,
    //     debug_string: None,
    //     commands: futures::future::try_join_all(responses).await?,
    // })
}
