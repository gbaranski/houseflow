use crate::extractors::UserID;
use crate::State;
use axum::extract::Extension;
use axum::Json;
use houseflow_types::errors::AuthError;
use houseflow_types::errors::FulfillmentError;
use houseflow_types::errors::ServerError;
use houseflow_types::fulfillment::execute::Request;
use houseflow_types::fulfillment::execute::Response;
use std::time::Instant;
use tracing::Level;

#[tracing::instrument(name = "Execute", skip(state), err)]
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

    Ok(Json(Response { frame: response }))
}
