use uuid::Uuid;
use std::sync::{Arc};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use std::collections::HashMap;

pub type DeviceSessions = HashMap<Uuid, Arc<Mutex<DeviceSession>>>;

pub struct DeviceSession {
    pub device_id: Uuid,
    stream: TcpStream,
    addr: SocketAddr,
}

impl DeviceSession {
    /// Waits for first CONNECT packet, if it doesn't return anything then it returns Error,
    /// otherwise makes new session
    async fn from_stream(stream: TcpStream, addr: SocketAddr) -> Self {
        Self {
            device_id: Uuid::new_v4(),
            stream,
            addr,
        }
    }

    async fn run(&mut self) {
        let mut buf = [0; 1024];
        loop {
            let n = match self.stream.read(&mut buf).await {
                // Connection closed
                Ok(n) if n == 0 => {
                    log::info!("Connection closed with device ID: {}, address: {}", self.device_id, self.addr);
                    return;
                },
                Ok(n) => n,
                Err(e) => {
                    log::error!("failed reading from socket, err: {}", e);
                    return;
                }
            };

            // Write the data back
            if let Err(e) = self.stream.write_all(&buf[0..n]).await {
                log::error!("failed to write to socket; err = {:?}", e);
                return;
            }
        }
    }
}

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
    device_sessions: Arc<Mutex<DeviceSessions>>,
}

impl TCPServer {
    pub fn new() -> Self {
        Self {
            device_sessions: Arc::new(Mutex::new(DeviceSessions::new())),
        }
    }

    pub async fn serve(&self) -> Result<(), ServerError> {
        let listener = TcpListener::bind("127.0.0.1:8080").await
            .map_err(|err| ServerError::FailBindListener(err))?;

        loop {
            let conn = listener.accept().await;
            if let Err(e) = conn {
                log::error!("failed accepting connection: {}", e);
                continue;
            }
            let (conn, addr) = conn.unwrap();
            log::info!("Connection accepted from address: {}", addr);

            let device_sessions = self.device_sessions.clone();

            tokio::spawn(async move {

                let device_session = DeviceSession::from_stream(conn, addr).await;
                let device_id = device_session.device_id;

                let device_session = Arc::new(Mutex::new(device_session)); // wrap it in Arc<Mutex> to allow being shared between threads

                log::info!("Session started with device ID: {}, address: {}", device_id, addr);

                // Add the device session into HashMap
                device_sessions.lock().await.insert(device_id.clone(), device_session.clone());
                device_session.lock().await.run().await;
                device_sessions.lock().await.remove(&device_id);
            });
        }
    }
}
