use serde::{Serialize, Deserialize};
pub use tcp_server::DeviceSessions;

mod types;
mod http_server;
mod tcp_server;

#[derive(Serialize, Deserialize)]
pub enum Error {
    InvalidPath(String),
    InvalidDeviceID(String),
}

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Self::InvalidDeviceID(err.to_string())
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    http_server::serve().await;


    Ok(())
}
