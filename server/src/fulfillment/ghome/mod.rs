use actix_web::{
    web::{self, Data, Json},
    HttpRequest,
};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    fulfillment::ghome::{
        self, IntentRequest, IntentRequestInput, IntentResponseBody, IntentResponseError,
    },
    token::AccessToken,
};

use crate::Sessions;

pub async fn on_webhook(
    Json(request): Json<IntentRequest>,
    http_request: HttpRequest,
    config: Data<Config>,
    db: web::Data<dyn Database>,
    sessions: web::Data<Sessions>,
) -> Result<web::Json<IntentResponseBody>, IntentResponseError> {
    let access_token =
        AccessToken::from_request(config.secrets.access_key.as_bytes(), &http_request)?;
    let input = request.inputs.first().unwrap();

    let body: Result<IntentResponseBody, IntentResponseError> = match input {
        IntentRequestInput::Sync(_) => {
            use ghome::sync;

            let user_devices = db
                .get_user_devices(&access_token.sub)
                .map_err(houseflow_db::Error::into_internal_server_error)?;

            let user_devices = user_devices
                .into_iter()
                .map(|device| {
                    let room = db
                        .get_room(&device.room_id)
                        .map_err(houseflow_db::Error::into_internal_server_error)?
                        .ok_or_else(|| {
                            IntentResponseError::InternalError(
                                houseflow_types::InternalServerError::Other(
                                    "couldn't find matching room".to_string(),
                                ),
                            )
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

                    Ok::<_, IntentResponseError>(payload)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let payload = sync::response::Payload {
                agent_user_id: access_token.sub.clone(),
                error_code: None,
                debug_string: None,
                devices: user_devices,
            };
            Ok(IntentResponseBody::Sync {
                request_id: request.request_id.clone(),
                payload,
            })
        }
        IntentRequestInput::Query(payload) => {
            use ghome::query;

            let db = &db;
            let access_token = &access_token;
            let sessions = &sessions;

            let device_responses = payload.devices.iter().map(|device| async move {
                if !db
                    .check_user_device_access(&access_token.sub, &device.id)
                    .map_err(houseflow_db::Error::into_internal_server_error)?
                {
                    return Err::<query::response::PayloadDevice, IntentResponseError>(
                        IntentResponseError::NoDevicePermission,
                    );
                }

                let sessions = sessions.lock().await;
                match sessions.get(&device.id) {
                    Some(session) => {
                        let query_frame = houseflow_types::lighthouse::proto::query::Frame {};
                        let response_frame = session
                            .send(crate::lighthouse::aliases::ActorQueryFrame::from(
                                query_frame,
                            ))
                            .await
                            .unwrap()?;
                        let response_frame: houseflow_types::lighthouse::proto::state::Frame =
                            response_frame.into();
                        Ok(query::response::PayloadDevice {
                            online: true,
                            status: ghome::DeviceStatus::Success,
                            error_code: None,
                            state: Some(response_frame.state),
                        })
                    }
                    None => Ok(query::response::PayloadDevice {
                        online: false,
                        status: ghome::DeviceStatus::Offline,
                        error_code: None,
                        state: None,
                    }),
                }
            });
            let device_responses = futures::future::try_join_all(device_responses).await?;
            let payload = query::response::Payload {
                error_code: None,
                debug_string: None,
                devices: device_responses,
            };

            Ok(IntentResponseBody::Query {
                request_id: request.request_id.clone(),
                payload,
            })
        }
        IntentRequestInput::Execute(_) => todo!(),
        IntentRequestInput::Disconnect => todo!(),
    };
    let body = body?;

    Ok(web::Json(body))
}
