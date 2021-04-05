use serde::{Serialize, Deserialize};
pub use tcp_server::DeviceSessions;
use tcp_server::TCPServer;

mod types;
mod http_server;
mod tcp_server;

#[derive(Serialize, Deserialize)]
pub enum Error {
    InvalidPath(String),
    InvalidDeviceID(String),
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    tokio::spawn(async {
        http_server::serve().await;
    });

    TCPServer::new().serve().await?;

    Ok(())
}
