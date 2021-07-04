use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest,
};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    fulfillment::query::{Request, ResponseBody, ResponseError},
    token::AccessToken,
};

use crate::Sessions;

#[post("/query")]
pub async fn on_query(
    request: Json<Request>,
    http_request: HttpRequest,
    config: Data<Config>,
    db: Data<dyn Database>,
    sessions: Data<Sessions>,
) -> Result<Json<ResponseBody>, ResponseError> {
    let access_token = AccessToken::from_request(config.secrets.access_key.as_bytes(), &http_request)?;
    if !db
        .check_user_device_access(&access_token.sub, &request.device_id)
        .await
        .map_err(houseflow_db::Error::into_internal_server_error)?
    {
        return Err(ResponseError::NoDevicePermission);
    }

    let sessions = sessions.lock().await;
    let session = sessions
        .get(&request.device_id)
        .ok_or(ResponseError::DeviceNotConnected)?;
    let response_frame = session
        .send(crate::lighthouse::aliases::ActorQueryFrame::from(
            request.frame.clone(),
        ))
        .await
        .unwrap()?;

    Ok(Json(ResponseBody {
        frame: response_frame.into(),
    }))
}
