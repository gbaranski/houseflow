use crate::{extractors::AccessToken, State};
use axum::{extract, response};
use houseflow_types::{
    errors::{AuthError, FulfillmentError, ServerError},
    fulfillment::internal::query::{Request, Response},
};

#[tracing::instrument(skip(state, access_token))]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    AccessToken(access_token): AccessToken,
    extract::Json(request): extract::Json<Request>,
) -> Result<response::Json<Response>, ServerError> {
    if !state
        .database
        .check_user_device_access(&access_token.sub, &request.device_id)?
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

    let frame = tokio::time::timeout(
        crate::fulfillment::QUERY_TIMEOUT,
        session.query(request.frame),
    )
    .await
    .map_err(|_| FulfillmentError::Timeout)??;

    Ok(response::Json(Response { frame }))
}
