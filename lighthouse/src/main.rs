pub(crate) use session::{
    Session,
    SessionStore,
    Request as SessionRequest,
    RequestReceiver,
    RequestSender,
};

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;


mod tcp_server;
mod http_server;
mod session;

#[tokio::main]
async fn main() {
    env_logger::init();

    let session_store = Arc::new(RwLock::new(HashMap::new()));
    let tcp_server = tcp_server::Server::new(session_store.clone()).await;
    tokio::select! {
        _ = tcp_server.run() => {

        },
        _ = http_server::serve(session_store.clone()) => {

        },
    };
    
    println!("Hello, world!");
}
