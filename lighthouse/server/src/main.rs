mod rpc;
pub mod tcp;
mod connection;

#[tokio::main]
async fn main() {
    env_logger::init();
    let connection_store = connection::Store::new();
    tcp::run("127.0.0.1", connection_store).await.expect("failed running tcp server");
}
