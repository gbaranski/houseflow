use crate::extractors::UserID;
use crate::State;
use axum::extract::Extension;
use axum::Json;
use futures::future::join_all;
use houseflow_types::errors::ServerError;
use houseflow_types::fulfillment::sync::Request;
use houseflow_types::fulfillment::sync::Response;
use houseflow_types::lighthouse;
use tracing::Level;

#[tracing::instrument(name = "Sync", skip(state, _request), err)]
pub async fn handle(
    Extension(state): Extension<State>,
    Json(_request): Json<Request>,
    UserID(user_id): UserID,
) -> Result<Json<Response>, ServerError> {
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
    let accessory_ids = accessories
        .clone()
        .map(|accessory| accessory.id)
        .collect::<Vec<_>>();

    tracing::event!(Level::INFO, ?accessory_ids, "Synchronized accessories");

    let response = Response {
        accessories: accessories.collect(),
    };

    Ok(Json(response))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test_utils::*;
//     use actix_web::{http, test};
//     use chrono::{Duration, Utc};
//     use houseflow_types::{
//         token::{AccessToken, AccessTokenPayload},
//         Device, UserStructure,
//     };
//
//     #[actix_rt::test]
//     async fn sync() {
//         let state = get_state();
//
//         let user = get_user();
//         let access_token = AccessToken::new(
//             state.config.secrets.access_key.as_bytes(),
//             AccessTokenPayload {
//                 sub: user.id.clone(),
//                 exp: Utc::now() + Duration::minutes(10),
//             },
//         );
//         state.database.add_user(&user).unwrap();
//
//         let structure_allow = get_structure();
//         let structure_deny = get_structure();
//         let room_allow = get_room(&structure_allow);
//         let room_deny = get_room(&structure_deny);
//         state.database.add_structure(&structure_allow).unwrap();
//         state.database.add_structure(&structure_deny).unwrap();
//         state.database.add_room(&room_allow).unwrap();
//         state.database.add_room(&room_deny).unwrap();
//         let devices_allow = std::iter::repeat_with(|| get_device(&room_allow))
//             .take(5)
//             .collect::<Vec<_>>();
//         let devices_deny = std::iter::repeat_with(|| get_device(&room_deny))
//             .take(5)
//             .collect::<Vec<_>>();
//
//         devices_allow
//             .iter()
//             .chain(devices_deny.iter())
//             .for_each(|device| state.database.add_device(&device).unwrap());
//
//         let user_structure = UserStructure {
//             structure_id: structure_allow.id.clone(),
//             user_id: user.id.clone(),
//             is_manager: false,
//         };
//         state.database.add_user_structure(&user_structure).unwrap();
//
//         let request = test::TestRequest::default()
//             .insert_header((
//                 http::header::AUTHORIZATION,
//                 format!("Bearer {}", access_token.to_string()),
//             ))
//             .to_http_request();
//         let response = on_sync(Json(Request {}), request, state.config, state.database)
//             .await
//             .unwrap()
//             .into_inner();
//         let sort_devices = |devices: Vec<Device>| {
//             devices.clone().sort_by(|a, b| a.id.cmp(&b.id));
//             devices
//         };
//         assert_eq!(
//             sort_devices(response.devices),
//             sort_devices(
//                 devices_allow
//                     .into_iter()
//                     .map(|device| Device {
//                         password_hash: None,
//                         ..device
//                     })
//                     .collect()
//             )
//         );
//     }
// }
