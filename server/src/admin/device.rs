use actix_web::{
    put,
    web::{Data, Json},
    HttpRequest,
};
use houseflow_config::server::Secrets;
use houseflow_db::Database;
use houseflow_types::UserAgent;
use houseflow_types::{
    admin::{AddDeviceRequest, AddDeviceResponse, AddDeviceResponseBody, AddDeviceResponseError},
    token::Token,
    Device,
};

const USER_AGENT: UserAgent = UserAgent::Internal;

#[put("/device")]
pub async fn on_add_device(
    add_device_request: Json<AddDeviceRequest>,
    http_request: HttpRequest,
    secrets: Data<Secrets>,
    db: Data<dyn Database>,
) -> Result<Json<AddDeviceResponse>, AddDeviceResponseError> {
    let add_device_request = add_device_request.0;
    let access_token = Token::from_request(&http_request)?;
    access_token.verify(&secrets.access_key, Some(&USER_AGENT))?;

    if !db
        .check_user_admin(access_token.user_id())
        .await
        .map_err(|err| AddDeviceResponseError::InternalError(err.to_string()))?
    {
        return Err(AddDeviceResponseError::UserNotAdmin);
    }

    let device = Device {
        id: rand::random(),
        room_id: add_device_request.room_id,
        password_hash: argon2::hash_encoded(
            add_device_request.password.as_bytes(),
            secrets.password_salt.as_bytes(),
            &argon2::Config::default(),
        )
        .unwrap(),
        device_type: add_device_request.device_type,
        traits: add_device_request.traits,
        name: add_device_request.name,
        will_push_state: add_device_request.will_push_state,
        model: add_device_request.model,
        hw_version: add_device_request.hw_version,
        sw_version: add_device_request.sw_version,
        attributes: add_device_request.attributes,
    };

    db.add_device(&device)
        .await
        .map_err(|err| AddDeviceResponseError::InternalError(err.to_string()))?;

    let response = AddDeviceResponseBody {
        device_id: device.id,
    };

    Ok(Json(AddDeviceResponse::Ok(response)))
}
