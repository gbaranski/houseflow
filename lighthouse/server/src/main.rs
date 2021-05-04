mod connection;
mod rpc;
pub mod tcp;

#[tokio::main]
async fn main() {
    env_logger::init();
    let connection_store = connection::Store::new();
    tokio::select! {
        _ = tcp::run("127.0.0.1:8080", connection_store.clone()) => {},
        _ = rpc::run("127.0.0.1:8081", connection_store.clone()) => {},
    };
    tcp::run("127.0.0.1", connection_store)
        .await
        .expect("failed running tcp server");
}
