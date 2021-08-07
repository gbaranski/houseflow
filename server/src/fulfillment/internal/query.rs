use std::time::Instant;

use crate::{extractors::UserID, State};
use axum::{extract, response};
use houseflow_types::{
    errors::{AuthError, FulfillmentError, ServerError},
    fulfillment::query::{Request, Response},
};
use tracing::Level;

#[tracing::instrument(name = "Query", skip(state), err)]
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
        crate::fulfillment::QUERY_TIMEOUT,
        session.query(request.frame),
    )
    .await
    .map_err(|_| FulfillmentError::Timeout)??;

    tracing::event!(
        Level::INFO,
        ?response,
        ms = %Instant::now().duration_since(before).as_millis(),
        "Queried device state"
    );

    Ok(response::Json(Response { frame: response }))
}
