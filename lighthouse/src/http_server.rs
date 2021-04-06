use super::types::*;
use crate::SessionStore;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::{http, Filter};

const MAX_CONTENT_LENGTH: u64 = 1024;

pub async fn serve(session_store: SessionStore) {
    let store_filter = warp::any().map(move || session_store.clone());

    let execute_path = warp::post()
        .and(warp::path("execute"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        // .and(warp::body::content_length_limit(MAX_CONTENT_LENGTH))
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(on_execute);

    let query_path = warp::get()
        .and(warp::path("query"))
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(warp::body::content_length_limit(MAX_CONTENT_LENGTH))
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(on_query);

    let routes = execute_path.or(query_path);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn on_execute(
    id: Uuid,
    request: ExecuteRequest,
    session_store: SessionStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    let resp = async {
        log::debug!("Attempt to get session");
        let session = session_store
            .lock()
            .await
            .get(&id)
            .ok_or(Error::DeviceNotFound)?
            .clone();

        log::debug!("Retrieved session");

        let response = session
            .lock()
            .await
            .send_execute(request)
            .await?;

        log::debug!("Sent execute");

        Ok::<_, Error>(response)
    };

    let resp = resp.await.unwrap_or_else(|err| ExecuteResponse {
        status: ResponseStatus::Error,
        states: HashMap::new(),
        error: Some(err),
    });

    Ok(warp::reply::json(&resp))
}

async fn on_query<'a>(
    id: Uuid,
    request: ExecuteRequest,
    session_store: SessionStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        format!("Received execute intent for deviceID: {}", id),
        http::StatusCode::OK,
    ))
}
