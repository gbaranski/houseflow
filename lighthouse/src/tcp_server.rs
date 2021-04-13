use tokio::{
    net::TcpListener,
    sync::mpsc,
};
use super::{Session, SessionStore};

pub struct Server {
    listener: TcpListener,
    sessions: SessionStore,
}

impl Server {    
    pub async fn new(sessions: SessionStore) -> Self {
        Self {
            listener: TcpListener::bind("127.0.0.1:8080").await.unwrap(),
            sessions, 
        }
    }

    pub async fn run(&self) {
        loop {
            let (stream, addr) = self.listener.accept().await.expect("failed accepting connection");

            let sessions = self.sessions.clone();
            tokio::spawn(async move {
                let (request_sender, request_receiver) = mpsc::channel(10);
                let session_id = addr.port().to_string();
                let session = Session::new(addr, session_id.clone());
                log::debug!("Connected with ID: {}", session_id);
                sessions.write().await.insert(session_id, request_sender);
                session.run(stream.into_split(), request_receiver).await;
            });
        }
    }
}
