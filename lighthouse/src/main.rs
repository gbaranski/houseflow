#[forbid(unsafe_code)] 
pub use houseflow_lighthouse::Error;

mod rpc;
mod server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let connections_store = server::connection::Store::new();

    tokio::select! {
        _ = server::run("127.0.0.1:8080", connections_store.clone()) => {

        },
        _ = rpc::run(connections_store) => {

        },
    };

    println!("Hello, world!");
}
