use crate::session;
use tokio::sync::oneshot;
use warp::Filter;

pub async fn serve(session_store: session::Store) {
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
    session_store: session::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let (tx, rx) = oneshot::channel();
    let session_request = session::Request::new(String::from("Hello world\n"), tx);
    session_store
        .send_to(&id, session_request)
        .await
        .unwrap_or_else(|_| panic!("Failed sending session_request"));

    let response = rx.await.expect("did not receive response");
    log::debug!("Received response: {}", response);
    Ok(warp::reply::json(&response))
}
