use server::Server;
pub use houseflow_lighthouse::Error;

mod server;
mod rpc;
mod session;

#[tokio::main]
async fn main() {
    env_logger::init();

    let session_store = session::Store::new();
    let server = Server::new(session_store.clone()).await;
    tokio::select! {
        _ = server.run() => {

        },
        _ = rpc::serve(session_store.clone()) => {

        },
    };
    
    println!("Hello, world!");
}
