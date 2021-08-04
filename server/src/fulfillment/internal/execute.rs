use crate::{extractors::AccessToken, Error, State};
use axum::{extract, response};
use houseflow_types::{
    fulfillment::execute::{Request, Response},
    FulfillmentError,
};
use tracing::Level;

#[tracing::instrument(skip(state, request))]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    extract::Json(request): extract::Json<Request>,
    AccessToken(access_token): AccessToken,
) -> Result<response::Json<Response>, Error> {
    if !state
        .database
        .check_user_device_access(&access_token.sub, &request.device_id)?
    {
        return Err(Error::NoDevicePermission);
    }

    tracing::event!(Level::INFO, user_id = %access_token.sub);

    let session = {
        let sessions = state.sessions.lock().unwrap();
        sessions
            .get(&request.device_id)
            .ok_or(FulfillmentError::DeviceNotConnected)?
            .clone()
    };

    let response_frame = session.execute(request.frame).await?;

    Ok(response::Json(Response {
        frame: response_frame.into(),
    }))
}
