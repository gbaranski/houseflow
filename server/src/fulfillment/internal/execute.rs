use actix_web::{post, web, HttpRequest};
use houseflow_config::server::Secrets;
use houseflow_db::Database;
use houseflow_types::UserAgent;
use houseflow_types::{
    fulfillment::{ExecuteRequest, ExecuteResponse, ExecuteResponseBody, ExecuteResponseError},
    token::Token,
};

use crate::Sessions;

#[post("/execute")]
pub async fn on_execute(
    execute_request: web::Json<ExecuteRequest>,
    http_request: HttpRequest,
    secrets: web::Data<Secrets>,
    db: web::Data<dyn Database>,
    sessions: web::Data<Sessions>,
) -> Result<web::Json<ExecuteResponse>, ExecuteResponseError> {
    let access_token = Token::from_request(&http_request)?;
    access_token.verify(&secrets.access_key, Some(&UserAgent::Internal))?;
    if !db
        .check_user_device_access(access_token.user_id(), &execute_request.device_id)
        .await
        .map_err(|err| ExecuteResponseError::InternalError(err.to_string()))?
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
