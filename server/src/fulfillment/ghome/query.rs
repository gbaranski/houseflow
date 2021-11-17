use crate::State;
use google_smart_home::query::request;
use google_smart_home::query::response;
use houseflow_types::device;
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
    let responses = payload.devices.iter().map(|device| async {
        let response = get_lighthouse_device(&state, &user_id, device).await?;
        Ok::<_, InternalError>((device.id.to_owned(), response))
    });
    let devices = futures::future::try_join_all(responses)
        .await?
        .into_iter()
        .collect();
    Ok(response::Payload {
        error_code: None,
        debug_string: None,
        devices,
    })
}

async fn get_lighthouse_device(
    state: &State,
    user_id: &user::ID,
    device: &request::PayloadDevice,
) -> Result<response::PayloadDevice, InternalError> {
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
