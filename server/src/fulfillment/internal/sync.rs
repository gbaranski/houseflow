use actix_web::{
    get,
    web::{Data, Json},
    HttpRequest,
};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    fulfillment::sync::{Request, ResponseBody, ResponseError},
    token::AccessToken,
};

#[get("/sync")]
pub async fn on_sync(
    _sync_request: Json<Request>,
    http_request: HttpRequest,
    config: Data<Config>,
    db: Data<dyn Database>,
) -> Result<Json<ResponseBody>, ResponseError> {
    let access_token =
        AccessToken::from_request(config.secrets.access_key.as_bytes(), &http_request)?;

    let devices = db
        .get_user_devices(&access_token.sub)
        .map_err(houseflow_db::Error::into_internal_server_error)?;
    let response = ResponseBody { devices };

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use actix_web::{http, test};
    use chrono::{Duration, Utc};
    use houseflow_types::{
        token::{AccessToken, AccessTokenPayload},
        Device, UserID, UserStructure,
    };

    async fn get_authorized_device(db: &dyn Database, user_id: &UserID) -> Device {
        let structure = get_structure();
        let room = get_room(&structure);
        let user_structure = UserStructure {
            structure_id: structure.id.clone(),
            user_id: user_id.clone(),
            is_manager: false,
        };
        let device = get_device(&room);
        db.add_structure(&structure).unwrap();
        db.add_room(&room).unwrap();
        db.add_device(&device).unwrap();
        db.add_user_structure(&user_structure).unwrap();
        device
    }

    async fn get_unauthorized_device(db: &dyn Database) -> Device {
        let structure = get_structure();
        let room = get_room(&structure);
        let device = get_device(&room);
        db.add_structure(&structure).unwrap();
        db.add_room(&room).unwrap();
        db.add_device(&device).unwrap();
        device
    }

    #[actix_rt::test]
    async fn sync() {
        use futures::future::join_all;
        use std::iter::repeat_with;

        let state = get_state();

        let user = get_user();
        let access_token = AccessToken::new(
            state.config.secrets.access_key.as_bytes(),
            AccessTokenPayload {
                sub: user.id.clone(),
                exp: Utc::now() + Duration::minutes(10),
            },
        );
        state.database.add_user(&user).unwrap();

        let mut authorized_devices: Vec<Device> = join_all(
            repeat_with(|| get_authorized_device(state.database.as_ref(), &user.id))
                .take(5)
                .collect::<Vec<_>>(),
        )
        .await;
        authorized_devices.sort_by_key(|device| device.id.clone());
        let authorized_devices = authorized_devices;

        let _: Vec<Device> = join_all(
            repeat_with(|| get_unauthorized_device(state.database.as_ref()))
                .take(10)
                .collect::<Vec<_>>(),
        )
        .await;

        let request_body = Request {};
        let request = test::TestRequest::get()
            .uri("/fulfillment/internal/sync")
            .insert_header((
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token.to_string()),
            ))
            .set_json(&request_body);
        let mut response = send_request_with_state::<ResponseBody>(request, &state).await;
        response.devices.sort_by_key(|device| device.id.clone());
        assert_eq!(response.devices, authorized_devices);
    }
}
