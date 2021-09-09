use crate::State;
use google_smart_home::sync::response;
use google_smart_home::device::Trait as GHomeDeviceTrait;
use google_smart_home::device::Type as GHomeDeviceType;
use houseflow_types::device::Trait as DeviceTrait;
use houseflow_types::device::Type as DeviceType;
use houseflow_types::errors::InternalError;
use houseflow_types::errors::ServerError;
use houseflow_types::user;

#[tracing::instrument(name = "Sync", skip(state), err)]
pub async fn handle(state: State, user_id: user::ID) -> Result<response::Payload, ServerError> {
    let user_devices = state.config.get_user_devices(&user_id);

    let user_devices = user_devices
        .into_iter()
        .map(|device_id| state.config.get_device(&device_id).unwrap())
        .map(|device| {
            let room = state
                .config
                .get_room(&device.room_id)
                .ok_or_else(|| InternalError::Other("couldn't find matching room".to_string()))?;
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
                        _ => todo!()
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
        .collect::<Result<Vec<_>, _>>()?;

    tracing::info!("Synced {} devices", user_devices.len());

    Ok(response::Payload {
        agent_user_id: user_id.to_string(),
        error_code: None,
        debug_string: None,
        devices: user_devices,
    })
}
