use std::time::Instant;

use crate::{extractors::UserID, State};
use axum::{extract::Extension, Json};
use houseflow_types::{
    errors::{AuthError, FulfillmentError, ServerError},
    fulfillment::query::{Request, Response},
};
use tracing::Level;

#[tracing::instrument(name = "Query", skip(state), err)]
pub async fn handle(
    Extension(state): Extension<State>,
    UserID(user_id): UserID,
    Json(request): Json<Request>,
) -> Result<Json<Response>, ServerError> {
    if state
        .config
        .get_permission(&request.device_id, &user_id)
        .is_none()
    {
        return Err(AuthError::NoDevicePermission.into());
    }

    let session = state
        .sessions
        .get(&request.device_id)
        .ok_or(FulfillmentError::DeviceNotConnected)?
        .clone();

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

    Ok(Json(Response { frame: response }))
}
