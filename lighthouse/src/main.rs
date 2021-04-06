use serde::{Serialize, Deserialize};
use tcp_server::TCPServer;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

pub use types::*;
pub(crate) use session::{Session, SessionStore};

mod types;
mod http_server;
mod tcp_server;
mod session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    let session_store: SessionStore = Arc::new(Mutex::new(HashMap::new()));

    // Clone session_store into this scope
    {
        let session_store = session_store.clone();
        tokio::spawn(async move {
            http_server::serve(session_store).await;
        });
    }

    TCPServer::new(session_store).serve().await?;

    Ok(())
}
