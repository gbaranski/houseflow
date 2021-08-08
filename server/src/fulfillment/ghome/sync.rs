use crate::State;
use google_smart_home::sync::response;
use houseflow_types::{
    errors::{InternalError, ServerError},
    UserID,
};

#[tracing::instrument(name = "Sync", skip(state), err)]
pub async fn handle(state: State, user_id: UserID) -> Result<response::Payload, ServerError> {
    let user_devices = state.database.get_user_devices(&user_id)?;

    let user_devices = user_devices
        .into_iter()
        .map(|device| {
            let room = state
                .database
                .get_room(&device.room_id)?
                .ok_or_else(|| InternalError::Other("couldn't find matching room".to_string()))?;
            let payload = response::PayloadDevice {
                id: device.id.to_string(),
                device_type: device.device_type.to_string(),
                traits: device.traits.iter().map(ToString::to_string).collect(),
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

    Ok(response::Payload {
        agent_user_id: user_id.to_string(),
        error_code: None,
        debug_string: None,
        devices: user_devices,
    })
}
