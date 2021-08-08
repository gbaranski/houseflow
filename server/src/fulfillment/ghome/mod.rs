mod execute;
mod query;
mod sync;

use crate::{extractors::UserID, State};
use axum::{extract::Extension, Json};
use google_smart_home::{Request, RequestInput, Response};
use houseflow_types::errors::ServerError;

#[tracing::instrument(skip(state), err)]
pub async fn on_webhook(
    Extension(state): Extension<State>,
    UserID(user_id): UserID,
    Json(request): Json<Request>,
) -> Result<Json<Response>, ServerError> {
    let input = request.inputs.first().unwrap();

    let body: Response = match input {
        RequestInput::Sync => Response::Sync(google_smart_home::sync::response::Response {
            request_id: request.request_id,
            payload: sync::handle(state, user_id).await?,
        }),
        RequestInput::Query(payload) => {
            Response::Query(google_smart_home::query::response::Response {
                request_id: request.request_id,
                payload: query::handle(state, user_id, payload).await?,
            })
        }
        RequestInput::Execute(payload) => {
            Response::Execute(google_smart_home::execute::response::Response {
                request_id: request.request_id,
                payload: execute::handle(state, user_id, payload).await?,
            })
        }
        RequestInput::Disconnect => todo!(),
    };

    Ok(Json(body))
}
