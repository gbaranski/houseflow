use actix_web::{post, web, HttpRequest};
use houseflow_config::server::Secrets;
use houseflow_db::Database;
use houseflow_fulfillment_types::{
    ExecuteRequest, ExecuteResponse, ExecuteResponseBody, ExecuteResponseError,
};
use houseflow_token::Token;
use houseflow_types::{DevicePermission, UserAgent};

use crate::Sessions;

const USER_AGENT: UserAgent = UserAgent::Internal;

const EXECUTE_PERMISSION: DevicePermission = DevicePermission {
    read: true,
    write: false,
    execute: true,
};

#[post("/execute")]
pub async fn on_execute(
    execute_request: web::Json<ExecuteRequest>,
    http_request: HttpRequest,
    secrets: web::Data<Secrets>,
    db: web::Data<dyn Database>,
    sessions: web::Data<Sessions>,
) -> Result<web::Json<ExecuteResponse>, ExecuteResponseError> {
    let access_token = Token::from_request(&http_request)?;
    access_token.verify(&secrets.access_key, Some(&USER_AGENT))?;
    if !db
        .check_user_device_permission(
            access_token.user_id(),
            &execute_request.device_id,
            &EXECUTE_PERMISSION,
        )
        .await?
    {
        return Err(ExecuteResponseError::NoDevicePermission);
    }

    let sessions = sessions.lock().await;
    let session = sessions
        .get(&execute_request.device_id)
        .ok_or(ExecuteResponseError::DeviceNotConnected)?;
    let response_frame = session
        .send(crate::lighthouse::aliases::ActorExecuteFrame::from(
            execute_request.frame.clone(),
        ))
        .await
        .unwrap()?;

    Ok(web::Json(ExecuteResponse::Ok(ExecuteResponseBody {
        frame: response_frame.into(),
    })))
}
