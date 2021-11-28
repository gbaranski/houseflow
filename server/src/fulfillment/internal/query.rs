use std::time::Instant;

use crate::extractors::UserID;
use crate::State;
use axum::extract::Extension;
use axum::Json;
use houseflow_types::errors::AuthError;
use houseflow_types::errors::FulfillmentError;
use houseflow_types::errors::ServerError;
use houseflow_types::fulfillment::query::Request;
use houseflow_types::fulfillment::query::Response;
use tracing::Level;

#[tracing::instrument(name = "Query", skip(state), err)]
pub async fn handle(
    Extension(state): Extension<State>,
    UserID(user_id): UserID,
    Json(request): Json<Request>,
) -> Result<Json<Response>, ServerError> {
    if state
        .config
        .get_permission(&request.structure_id, &user_id)
        .is_none()
    {
        return Err(AuthError::NoStructurePermission.into());
    }

    let session = state
        .sessions
        .get(&request.device_id)
        .ok_or(FulfillmentError::HubNotConnected)?
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
