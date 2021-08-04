use crate::{extractors::AccessToken, Error, State};
use axum::{extract, response};
use houseflow_types::{
    fulfillment::query::{Request, Response},
    FulfillmentError,
};

#[tracing::instrument(skip(state, access_token))]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    AccessToken(access_token): AccessToken,
    extract::Json(request): extract::Json<Request>,
) -> Result<response::Json<Response>, Error> {
    if !state
        .database
        .check_user_device_access(&access_token.sub, &request.device_id)?
    {
        return Err(FulfillmentError::NoDevicePermission.into());
    }

    let session = {
        let sessions = state.sessions.lock().unwrap();
        sessions
            .get(&request.device_id)
            .ok_or(FulfillmentError::DeviceNotConnected)?
            .clone()
    };

    let state_frame = session.query(request.frame).await?;

    Ok(response::Json(Response {
        frame: state_frame.into(),
    }))
}
