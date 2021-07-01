use actix_web::{post, web, HttpRequest};
use houseflow_config::server::Secrets;
use houseflow_db::Database;
use houseflow_types::UserAgent;
use houseflow_types::{
    fulfillment::{QueryRequest, QueryResponse, QueryResponseBody, QueryResponseError},
    token::Token,
};

use crate::Sessions;

const USER_AGENT: UserAgent = UserAgent::Internal;

#[post("/query")]
pub async fn on_query(
    query_request: web::Json<QueryRequest>,
    http_request: HttpRequest,
    secrets: web::Data<Secrets>,
    db: web::Data<dyn Database>,
    sessions: web::Data<Sessions>,
) -> Result<web::Json<QueryResponse>, QueryResponseError> {
    let access_token = Token::from_request(&http_request)?;
    access_token.verify(&secrets.access_key, Some(&USER_AGENT))?;
    if !db
        .check_user_device_access(access_token.user_id(), &query_request.device_id)
        .await
        .map_err(|err| QueryResponseError::InternalError(err.to_string()))?
    {
        return Err(QueryResponseError::NoDevicePermission);
    }

    let sessions = sessions.lock().await;
    let session = sessions
        .get(&query_request.device_id)
        .ok_or(QueryResponseError::DeviceNotConnected)?;
    let response_frame = session
        .send(crate::lighthouse::aliases::ActorQueryFrame::from(
            query_request.frame.clone(),
        ))
        .await
        .unwrap()?;

    Ok(web::Json(QueryResponse::Ok(QueryResponseBody {
        frame: response_frame.into(),
    })))
}
