use warp::Filter;
use crate::server::connection;

pub async fn run(connection_store: connection::Store) {
    let store_filter = warp::any().map(move || connection_store.clone());

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
    connection_store: connection::Store
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn_request = connection::Request::new(Vec::from("hello world"));
    let conn_resp = connection_store
        .send_request(&id, conn_request)
        .await
        .expect("failed sending request");

    log::debug!("Received response: {}", conn_resp);
    Ok(warp::reply::json(&"dshsdahads".to_string()))
}
