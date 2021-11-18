use super::homie::get_homie_device_by_id;
use crate::State;
use google_smart_home::query::request;
use google_smart_home::query::response;
use homie_controller::Device;
use houseflow_types::device;
use houseflow_types::errors::InternalError;
use houseflow_types::lighthouse;
use houseflow_types::user;
use serde_json::Map;
use serde_json::Number;
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

#[tracing::instrument(name = "Query", skip(state), err)]
pub async fn handle(
    state: State,
    user_id: user::ID,
    payload: &request::Payload,
) -> Result<response::Payload, InternalError> {
    let devices = if let Some(homie_controller) = state.homie_controllers.get(&user_id) {
        get_homie_devices(&homie_controller.devices(), &payload.devices)
    } else {
        get_lighthouse_devices(&state, &user_id, &payload.devices).await?
    };
    Ok(response::Payload {
        error_code: None,
        debug_string: None,
        devices,
    })
}

fn get_homie_devices(
    devices: &HashMap<String, Device>,
    request_devices: &[request::PayloadDevice],
) -> HashMap<String, response::PayloadDevice> {
    request_devices
        .iter()
        .map(|device| {
            let response = get_homie_device(devices, device);
            (device.id.to_owned(), response)
        })
        .collect()
}

fn get_homie_device(
    devices: &HashMap<String, Device>,
    request_device: &request::PayloadDevice,
) -> response::PayloadDevice {
    if let Some((device, node)) = get_homie_device_by_id(devices, &request_device.id) {
        if device.state == homie_controller::State::Ready
            || device.state == homie_controller::State::Sleeping
        {
            let mut state = Map::new();

            if let Some(on) = node.properties.get("on") {
                if let Ok(value) = on.value() {
                    state.insert("on".to_string(), Value::Bool(value));
                }
            }
            if let Some(brightness) = node.properties.get("brightness") {
                if let Ok(value) = brightness.value::<i64>() {
                    // TODO: Scale to percentage.
                    state.insert("brightness".to_string(), Value::Number(value.into()));
                }
            }
            if let Some(on) = node.properties.get("temperature") {
                if let Ok(value) = on.value::<f64>() {
                    if let Some(finite_number) = Number::from_f64(value) {
                        state.insert(
                            "thermostatTemperatureAmbient".to_string(),
                            Value::Number(finite_number),
                        );
                    }
                }
            }

            response::PayloadDevice {
                state,
                status: response::PayloadDeviceStatus::Success,
                error_code: None,
            }
        } else {
            response::PayloadDevice {
                state: Default::default(),
                status: response::PayloadDeviceStatus::Offline,
                error_code: Some("offline".to_string()),
            }
        }
    } else {
        response::PayloadDevice {
            status: response::PayloadDeviceStatus::Error,
            state: Default::default(),
            error_code: Some("deviceNotFound".to_string()),
        }
    }
}

async fn get_lighthouse_devices(
    state: &State,
    user_id: &user::ID,
    devices: &[request::PayloadDevice],
) -> Result<HashMap<String, response::PayloadDevice>, InternalError> {
    let responses = devices.iter().map(|device| async {
        let response = get_lighthouse_device(state, user_id, device).await?;
        Ok::<_, InternalError>((device.id.to_owned(), response))
    });
    Ok(futures::future::try_join_all(responses)
        .await?
        .into_iter()
        .collect())
}

async fn get_lighthouse_device(
    state: &State,
    user_id: &user::ID,
    device: &request::PayloadDevice,
) -> Result<response::PayloadDevice, InternalError> {
    // TODO: Return error rather than panicking.
    let device_id = device::ID::from_str(&device.id).expect("invalid device ID");

    if state.config.get_permission(&device_id, user_id).is_none() {
        return Ok(response::PayloadDevice {
            status: response::PayloadDeviceStatus::Error,
            state: Default::default(),
            error_code: Some(String::from("authFailure")),
        });
    }
    let session = match state.sessions.get(&device_id) {
        Some(session) => session.clone(),
        None => {
            return Ok(response::PayloadDevice {
                state: Default::default(),
                status: response::PayloadDeviceStatus::Offline,
                error_code: Some(String::from("offline")),
            })
        }
    };

    let request = lighthouse::query::Frame {};
    let response =
        match tokio::time::timeout(crate::fulfillment::EXECUTE_TIMEOUT, session.query(request))
            .await
        {
            Ok(val) => val?,
            Err(_) => {
                return Ok(response::PayloadDevice {
                    status: response::PayloadDeviceStatus::Offline,
                    state: Default::default(),
                    error_code: Some(String::from("offline")),
                })
            }
        };

    tracing::info!(state = %serde_json::to_string(&response.state).unwrap(), "Queried device state");

    Ok(response::PayloadDevice {
        status: response::PayloadDeviceStatus::Success,
        error_code: None,
        state: response.state,
    })
}
