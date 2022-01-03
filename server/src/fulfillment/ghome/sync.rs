use crate::State;
use futures::future::join_all;
use google_smart_home::device;
use google_smart_home::sync::response;
use houseflow_types::accessory;
use houseflow_types::accessory::manufacturers;
use houseflow_types::errors::ServerError;
use houseflow_types::lighthouse;
use houseflow_types::user;
use serde_json::json;

#[tracing::instrument(name = "Sync", skip(state), err)]
pub async fn handle(state: State, user_id: user::ID) -> Result<response::Payload, ServerError> {
    let user_hubs = state.config.get_user_hubs(&user_id);
    let accessories = user_hubs.iter().map(|hub| async {
        let hub = state.sessions.get(&hub.id).unwrap();
        let response = hub.hub_query(lighthouse::HubQueryFrame {}).await?;
        Ok::<_, ServerError>(response.accessories)
    });
    let accessories = join_all(accessories)
        .await
        .into_iter()
        .map(Result::unwrap) // TODO: Remove this unwrap
        .flatten();
    let devices = accessories
        .map(|accessory| {
            let (r#type, traits, attributes) = match accessory.r#type {
                accessory::Type::XiaomiMijia(accessory_type) => match accessory_type {
                    manufacturers::XiaomiMijia::HygroThermometer => (
                        device::Type::Thermostat,
                        vec![device::Trait::TemperatureControl],
                        json!({"queryOnlyTemperatureControl": true}),
                    ),
                    _ => unimplemented!(),
                },
                accessory::Type::Houseflow(accessory_type) => match accessory_type {
                    manufacturers::Houseflow::Garage => (
                        device::Type::Garage,
                        vec![device::Trait::OpenClose],
                        json!({}),
                    ),
                    manufacturers::Houseflow::Gate => (
                        device::Type::Garage,
                        vec![device::Trait::OpenClose],
                        json!({}),
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
                    name: accessory.name,
                    nicknames: None,
                },
                will_report_state: false,
                notification_supported_by_agent: false, // not sure about that
                room_hint: Some(accessory.room_name),
                device_info: Some(response::PayloadDeviceInfo {
                    manufacturer: Some("houseflow".to_string()),
                    model: None,
                    hw_version: None,
                    sw_version: None,
                }),
                attributes: attributes.as_object().unwrap().clone(),
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
