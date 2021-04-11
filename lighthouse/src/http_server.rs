use super::{RequestChannel, SessionRequest, SessionStore};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use warp::{http, Filter};

const MAX_CONTENT_LENGTH: u64 = 1024;

pub async fn serve(session_store: SessionStore) {
    let store_filter = warp::any().map(move || session_store.clone());

    let execute_path = warp::post()
        .and(warp::path("execute"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(on_execute);

    warp::serve(execute_path).run(([127, 0, 0, 1], 3030)).await;
}

async fn on_execute(
    id: String,
    session_store: SessionStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    let (tx, rx) = oneshot::channel();
    let session_request = SessionRequest::new(String::from("Hello world\n"), tx);
    session_store
        .read()
        .await
        .get(&id)
        .expect("session found")
        .send(session_request)
        .await
        .unwrap_or_else(|_| panic!(":wu"));

    let response = rx.await.expect("did not receive response");
    log::debug!("Received response: {}", response);
    Ok(warp::reply::json(&response))
}
