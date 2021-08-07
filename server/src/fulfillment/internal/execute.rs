use crate::{extractors::UserID, State};
use axum::{extract, response};
use houseflow_types::{
    errors::{AuthError, FulfillmentError, ServerError},
    fulfillment::execute::{Request, Response},
};
use std::time::Instant;
use tracing::Level;

#[tracing::instrument(name = "Execute", skip(state), err)]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    UserID(user_id): UserID,
    extract::Json(request): extract::Json<Request>,
) -> Result<response::Json<Response>, ServerError> {
    if !state
        .database
        .check_user_device_access(&user_id, &request.device_id)?
    {
        return Err(AuthError::NoDevicePermission.into());
    }

    let session = {
        let sessions = state.sessions.lock().unwrap();
        sessions
            .get(&request.device_id)
            .ok_or(FulfillmentError::DeviceNotConnected)?
            .clone()
    };

    let before = Instant::now();
    let response = tokio::time::timeout(
        crate::fulfillment::EXECUTE_TIMEOUT,
        session.execute(request.frame),
    )
    .await
    .map_err(|_| FulfillmentError::Timeout)??;

    tracing::event!(
        Level::INFO,
        response = ?response,
        ms = %Instant::now().duration_since(before).as_millis(),
        "Executed command on device"
    );

    Ok(response::Json(Response { frame: response }))
}
