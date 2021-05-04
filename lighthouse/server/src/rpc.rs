use crate::connection;
use lighthouse_proto::frame::{self, ClientID};
use std::net::ToSocketAddrs;
use warp::Filter;

const CONTENT_LENGTH_MAX: usize = 4096;

pub async fn run(addr: impl ToSocketAddrs, connection_store: connection::Store) {
    let addr = addr
        .to_socket_addrs()
        .expect("RPC has invalid address")
        .nth(0)
        .unwrap();

    log::info!("Starting RPC server at address: {:?}", addr);
    let store_filter = warp::any().map(move || connection_store.clone());

    let execute_path = warp::post()
        .and(warp::path("execute"))
        .and(warp::path::param::<ClientID>())
        .and(warp::body::content_length_limit(CONTENT_LENGTH_MAX as u64))
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(on_execute);

    warp::serve(execute_path).run(addr).await;
}

async fn on_execute(
    _client_id: ClientID,
    _frame: frame::execute::Frame,
    _connection_store: connection::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO :impl that
//     let conn_resp = connection_store
//         .send_request(&client_id, Frame::Execute(frame))
//         .await
//         .expect("failed sending request");
// 
//     log::debug!("Received response: {:?}", conn_resp);
    Ok(warp::reply::json(&"dshsdahads".to_string()))
}
