use crate::{extractors::AccessToken, Error, InternalError, State};
use axum::{extract, response};
use houseflow_types::{
    fulfillment::ghome::{self, Request, RequestInput, Response},
    DeviceID, DeviceStatus, FulfillmentError,
};
use tracing::Level;

#[tracing::instrument(skip(state), fields(request = ?request))]
pub async fn on_webhook(
    extract::Extension(state): extract::Extension<State>,
    extract::Json(request): extract::Json<Request>,
    AccessToken(access_token): AccessToken,
) -> Result<response::Json<Response>, Error> {
    let input = request.inputs.first().unwrap();
    tracing::event!(Level::INFO, intent = %input, user_id = %access_token.sub);

    let body: Result<Response, Error> = match input {
        RequestInput::Sync => {
            use ghome::sync;

            let user_devices = state.database.get_user_devices(&access_token.sub)?;

            let user_devices = user_devices
                .into_iter()
                .map(|device| {
                    let room = state.database.get_room(&device.room_id)?.ok_or_else(|| {
                        InternalError::Other("couldn't find matching room".to_string())
                    })?;
                    let payload = sync::response::PayloadDevice {
                        id: device.id,
                        device_type: device.device_type,
                        traits: device.traits,
                        name: ghome::sync::response::PayloadDeviceName {
                            default_names: None,
                            name: device.name,
                            nicknames: None,
                        },
                        will_report_state: device.will_push_state,
                        notification_supported_by_agent: false, // not sure about that
                        room_hint: Some(room.name),
                        device_info: Some(sync::response::PayloadDeviceInfo {
                            manufacturer: Some("houseflow".to_string()),
                            model: None,
                            hw_version: Some(device.hw_version),
                            sw_version: Some(device.sw_version),
                        }),
                        attributes: Some(device.attributes),
                        custom_data: None,
                        other_device_ids: None,
                    };

                    Ok::<_, Error>(payload)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let payload = sync::response::Payload {
                agent_user_id: access_token.sub.clone(),
                error_code: None,
                debug_string: None,
                devices: user_devices,
            };
            Ok(Response::Sync {
                request_id: request.request_id.clone(),
                payload,
            })
        }
        RequestInput::Query(payload) => {
            use ghome::query;

            // TODO: remove that as soon as Rust 2021 edition will be ther
            let database = &state.database;
            let access_token = &access_token;
            let sessions = &state.sessions;

            let device_responses = payload.devices.iter().map(|device| async move {
                if !database.check_user_device_access(&access_token.sub, &device.id)? {
                    return Err::<(DeviceID, query::response::PayloadDevice), Error>(
                        FulfillmentError::NoDevicePermission.into(),
                    );
                }

                let sessions = sessions.lock().unwrap();
                match sessions.get(&device.id) {
                    Some(session) => {
                        let query_frame = houseflow_types::lighthouse::proto::query::Frame {};
                        let response = session.query(query_frame).await?;
                        Ok((
                            device.id.clone(),
                            query::response::PayloadDevice {
                                online: true,
                                status: ghome::DeviceStatus::Success,
                                error_code: None,
                                state: Some(response.state),
                            },
                        ))
                    }
                    None => Ok((
                        device.id.clone(),
                        query::response::PayloadDevice {
                            online: false,
                            status: ghome::DeviceStatus::Offline,
                            error_code: None,
                            state: None,
                        },
                    )),
                }
            });
            let device_responses = futures::future::try_join_all(device_responses).await?;
            let mut map = std::collections::HashMap::new();
            for (device_id, response) in device_responses {
                map.insert(device_id, response);
            }

            let payload = query::response::Payload {
                error_code: None,
                debug_string: None,
                devices: map,
            };

            Ok(Response::Query {
                request_id: request.request_id.clone(),
                payload,
            })
        }
        RequestInput::Execute(payload) => {
            use ghome::execute;

            let requests = payload
                .commands
                .iter()
                .flat_map(|cmd| cmd.execution.iter().zip(cmd.devices.iter()));

            let database = &state.database;
            let sessions = &state.sessions;
            let access_token = &access_token;
            let responses = requests.map(|(exec, device)| async move {
                if !database.check_user_device_access(&access_token.sub, &device.id)? {
                    return Err::<_, Error>(FulfillmentError::NoDevicePermission.into());
                }
                match sessions.lock().unwrap().get(&device.id) {
                    Some(session) => {
                        let execute_frame = houseflow_types::lighthouse::proto::execute::Frame {
                            id: rand::random(),
                            command: exec.command.clone(),
                            params: exec.params.clone(),
                        };

                        let response = session.execute(execute_frame).await?;

                        Ok(match response.status {
                            DeviceStatus::Success => execute::response::PayloadCommand {
                                ids: vec![device.id.clone()],
                                status: ghome::DeviceStatus::Success,
                                states: response.state,
                                error_code: None,
                            },
                            DeviceStatus::Error(err) => execute::response::PayloadCommand {
                                ids: vec![device.id.clone()],
                                status: ghome::DeviceStatus::Error,
                                states: response.state,
                                error_code: Some(err.to_string()),
                            },
                        })
                    }
                    None => Ok(execute::response::PayloadCommand {
                        ids: vec![device.id.clone()],
                        status: ghome::DeviceStatus::Offline,
                        states: Default::default(),
                        error_code: None,
                    }),
                }
            });
            let payload = execute::response::Payload {
                error_code: None,
                debug_string: None,
                commands: futures::future::try_join_all(responses).await?,
            };
            Ok(Response::Execute {
                request_id: request.request_id,
                payload,
            })
        }
        RequestInput::Disconnect => todo!(),
    };
    let body = body?;

    Ok(response::Json(body))
}
