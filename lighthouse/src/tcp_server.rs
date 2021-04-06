use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use std::collections::HashMap;
use super::session::{Session, SessionStore};

#[derive(Debug)]
pub enum ServerError {
    FailBindListener(tokio::io::Error),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ServerError::FailBindListener(err) => format!("failed binding listener: {}", err),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for ServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

pub struct TCPServer {
    session_store: SessionStore,
}

impl TCPServer {
    pub fn new(session_store: SessionStore) -> Self {
        Self {
            session_store,
        }
    }

    pub async fn serve(&mut self) -> Result<(), ServerError> {
        let listener = TcpListener::bind("127.0.0.1:8080").await
            .map_err(|err| ServerError::FailBindListener(err))?;

        loop {
            let conn = listener.accept().await;
            if let Err(e) = conn {
                log::error!("failed accepting connection: {}", e);
                continue;
            }
            let (stream, addr) = conn.unwrap();
            log::info!("Connection accepted from address: {}", addr);

            let session_store = self.session_store.clone();

            tokio::spawn(async move {
                let session = Session::new(addr, session_store, stream).await;
                log::info!("Session started with device ID: {}, address: {}", session.client_id, addr);
                session.run().await;
            });
        }
    }
}
