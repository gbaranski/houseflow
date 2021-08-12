use crate::State;
use google_smart_home::sync::response;
use houseflow_types::{
    errors::{InternalError, ServerError},
    UserID,
};

#[tracing::instrument(name = "Sync", skip(state), err)]
pub async fn handle(state: State, user_id: UserID) -> Result<response::Payload, ServerError> {
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
                device_type: format!(
                    "{}.{}",
                    super::TYPE_PREFIX,
                    device.device_type.to_string().to_uppercase()
                ),
                traits: device
                    .traits
                    .iter()
                    .map(ToString::to_string)
                    .map(|name| format!("{}.{}", super::TRAIT_PREFIX, name))
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
                attributes: Some(device.attributes),
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
