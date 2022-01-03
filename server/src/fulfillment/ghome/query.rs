use crate::State;
use google_smart_home::query::request;
use google_smart_home::query::response;
use houseflow_types::accessory;
use houseflow_types::errors::InternalError;
use houseflow_types::lighthouse;
use houseflow_types::user;
use std::str::FromStr;

#[tracing::instrument(name = "Query", skip(state), err)]
pub async fn handle(
    state: State,
    user_id: user::ID,
    payload: &request::Payload,
) -> Result<response::Payload, InternalError> {
    let sessions = &state.sessions;
    let config = &state.config;
    let user_id = &user_id;

    let responses = payload.devices.iter().map(|device| async move {
        let response = (|| async {
            let device_id = accessory::ID::from_str(&device.id).expect("invalid device ID");
            if config.get_permission(&device_id, user_id).is_none() {
                return Ok::<_, InternalError>(response::PayloadDevice {
                    status: response::PayloadDeviceStatus::Error,
                    state: Default::default(),
                    error_code: Some(String::from("authFailure")),
                });
            }
            let session = match sessions.get(&device_id) {
                Some(session) => session.clone(),
                None => {
                    return Ok(response::PayloadDevice {
                        state: Default::default(),
                        status: response::PayloadDeviceStatus::Offline,
                        error_code: Some(String::from("offline")),
                    })
                }
            };

            let response = match tokio::time::timeout(
                crate::fulfillment::QUERY_TIMEOUT,
                session.accessory_query(lighthouse::AccessoryQueryFrame {}),
            )
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
                state: serde_json::json!({}).as_object().unwrap().clone(), // TODO: implement states
            })
        })();
        response
            .await
            .map(|response| (device.id.to_owned(), response))
    });
    Ok(response::Payload {
        error_code: None,
        debug_string: None,
        devices: futures::future::try_join_all(responses)
            .await?
            .into_iter()
            .collect(),
    })
}
