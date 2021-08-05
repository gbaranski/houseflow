use crate::{extractors::AccessToken, State};
use axum::{extract, response};
use houseflow_types::{
    errors::{AuthError, FulfillmentError, ServerError},
    fulfillment::internal::execute::{Request, Response},
};
use tracing::Level;

#[tracing::instrument(skip(state, request))]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    extract::Json(request): extract::Json<Request>,
    AccessToken(access_token): AccessToken,
) -> Result<response::Json<Response>, ServerError> {
    if !state
        .database
        .check_user_device_access(&access_token.sub, &request.device_id)?
    {
        return Err(AuthError::NoDevicePermission.into());
    }

    tracing::event!(Level::INFO, user_id = %access_token.sub);

    let session = {
        let sessions = state.sessions.lock().unwrap();
        sessions
            .get(&request.device_id)
            .ok_or(FulfillmentError::DeviceNotConnected)?
            .clone()
    };

    let response_frame = tokio::time::timeout(
        crate::fulfillment::EXECUTE_TIMEOUT,
        session.execute(request.frame),
    )
    .await
    .map_err(|_| FulfillmentError::Timeout)??;

    Ok(response::Json(Response {
        frame: response_frame.into(),
    }))
}
