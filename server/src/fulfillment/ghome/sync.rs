use std::collections::HashMap;

use crate::State;
use google_smart_home::device::Trait as GHomeDeviceTrait;
use google_smart_home::device::Type as GHomeDeviceType;
use google_smart_home::sync::response;
use google_smart_home::sync::response::PayloadDevice;
use homie_controller::Device;
use homie_controller::Node;
use houseflow_types::device::Trait as DeviceTrait;
use houseflow_types::device::Type as DeviceType;
use houseflow_types::errors::InternalError;
use houseflow_types::errors::ServerError;
use houseflow_types::user;
use serde_json::Map;
use serde_json::Value;

#[tracing::instrument(name = "Sync", skip(state), err)]
pub async fn handle(state: State, user_id: user::ID) -> Result<response::Payload, ServerError> {
    let user_devices = if let Some(homie_controller) = state.homie_controllers.get(&user_id) {
        homie_devices_to_google_home(&homie_controller.devices())
    } else {
        let user_devices = state.config.get_user_devices(&user_id);

        user_devices
            .into_iter()
            .map(|device_id| state.config.get_device(&device_id).unwrap())
            .map(|device| {
                let room = state.config.get_room(&device.room_id).ok_or_else(|| {
                    InternalError::Other("couldn't find matching room".to_string())
                })?;
                let payload = response::PayloadDevice {
                    id: device.id.to_string(),
                    device_type: match device.device_type {
                        DeviceType::Garage => GHomeDeviceType::Garage,
                        DeviceType::Gate => GHomeDeviceType::Gate,
                        DeviceType::Light => GHomeDeviceType::Light,
                        _ => todo!(),
                    },
                    traits: device
                        .traits
                        .iter()
                        .map(|t| match t {
                            DeviceTrait::OnOff => GHomeDeviceTrait::OnOff,
                            DeviceTrait::OpenClose => GHomeDeviceTrait::OpenClose,
                            _ => todo!(),
                        })
                        .collect(),
                    name: response::PayloadDeviceName {
                        default_names: None,
                        name: device.name,
                        nicknames: None,
                    },
                    will_report_state: device.will_push_state,
                    notification_supported_by_agent: false, // not sure about that
                    room_hint: Some(room.name),
                    device_info: Some(response::PayloadDeviceInfo {
                        manufacturer: Some("houseflow".to_string()),
                        model: None,
                        hw_version: Some(device.hw_version.to_string()),
                        sw_version: Some(device.sw_version.to_string()),
                    }),
                    attributes: device.attributes,
                    custom_data: None,
                    other_device_ids: None,
                };

                Ok::<_, ServerError>(payload)
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    tracing::info!(
        "Synced {} devices: {}",
        user_devices.len(),
        serde_json::to_string_pretty(&user_devices).unwrap(),
    );

    Ok(response::Payload {
        agent_user_id: user_id.to_string(),
        error_code: None,
        debug_string: None,
        devices: user_devices,
    })
}

fn homie_devices_to_google_home(devices: &HashMap<String, Device>) -> Vec<PayloadDevice> {
    let mut google_home_devices = vec![];
    for device in devices.values() {
        if device.state == homie_controller::State::Ready
            || device.state == homie_controller::State::Sleeping
        {
            for node in device.nodes.values() {
                if let Some(google_home_device) = homie_node_to_google_home(device, node) {
                    google_home_devices.push(google_home_device);
                }
            }
        }
    }
    google_home_devices
}

fn homie_node_to_google_home(device: &Device, node: &Node) -> Option<PayloadDevice> {
    let id = format!("{}/{}", device.id, node.id);
    let mut traits = vec![];
    let mut attributes = Map::new();
    let mut device_type = None;
    if node.properties.contains_key("on") {
        device_type = Some(GHomeDeviceType::Switch);
        traits.push(GHomeDeviceTrait::OnOff);
    }
    if node.properties.contains_key("brightness") {
        if node.properties.contains_key("on") {
            device_type = Some(GHomeDeviceType::Light);
        }
        traits.push(GHomeDeviceTrait::Brightness);
    }
    if node.properties.contains_key("temperature") {
        device_type = Some(GHomeDeviceType::Thermostat);
        traits.push(GHomeDeviceTrait::TemperatureSetting);
        attributes.insert(
            "availableThermostatModes".to_string(),
            Value::Array(vec![Value::String("off".to_string())]),
        );
        attributes.insert(
            "thermostatTemperatureUnit".to_string(),
            Value::String("C".to_string()),
        );
        attributes.insert("queryOnlyTemperatureSetting".to_string(), Value::Bool(true));
    }

    let device_name = device.name.clone().unwrap_or_else(|| device.id.clone());
    let node_name = node.name.clone().unwrap_or_else(|| node.id.clone());
    Some(response::PayloadDevice {
        id,
        device_type: device_type?,
        traits,
        name: response::PayloadDeviceName {
            default_names: None,
            name: format!("{} {}", device_name, node_name),
            nicknames: Some(vec![node_name]),
        },
        device_info: None,
        will_report_state: false,
        notification_supported_by_agent: false,
        room_hint: None,
        attributes,
        custom_data: None,
        other_device_ids: None,
    })
}
