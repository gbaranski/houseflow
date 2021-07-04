use actix_web::{
    put,
    web::{Data, Json},
    HttpRequest,
};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    admin::device::add::{Request, ResponseBody, ResponseError},
    token::AccessToken,
    Device,
};

#[put("/device")]
pub async fn on_add(
    Json(request): Json<Request>,
    http_request: HttpRequest,
    config: Data<Config>,
    db: Data<dyn Database>,
) -> Result<Json<ResponseBody>, ResponseError> {
    let access_token = AccessToken::from_request(&config.secrets.access_key, &http_request)?;

    if !db
        .check_user_admin(&access_token.sub)
        .await
        .map_err(houseflow_db::Error::into_internal_server_error)?
    {
        return Err(ResponseError::UserNotAdmin);
    }

    let device = Device {
        id: rand::random(),
        room_id: request.room_id,
        password_hash: argon2::hash_encoded(
            request.password.as_bytes(),
            config.secrets.password_salt.as_bytes(),
            &argon2::Config::default(),
        )
        .unwrap(),
        device_type: request.device_type,
        traits: request.traits,
        name: request.name,
        will_push_state: request.will_push_state,
        model: request.model,
        hw_version: request.hw_version,
        sw_version: request.sw_version,
        attributes: request.attributes,
    };

    db.add_device(&device)
        .await
        .map_err(houseflow_db::Error::into_internal_server_error)?;

    Ok(Json(ResponseBody {
        device_id: device.id,
    }))
}
