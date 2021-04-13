use super::session;
use session::Session;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpListener,
    sync::mpsc,
};

const REQUEST_CHANNEL_BUFFER: usize = 32;

#[derive(Clone)]
pub struct Server {
    sessions: session::Store,
}

impl Server {
    pub async fn new(sessions: session::Store) -> Self {
        Self { sessions }
    }

    pub async fn run(self) {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

        loop {
            let (stream, addr) = listener
                .accept()
                .await
                .expect("failed accepting connection");

            // That is possible because Self implements Clone trait,
            // and that is possible because we're using session::Store which is behind thread-safe reference counting pointer `Arc`
            let cloned_self = self.clone();

            tokio::spawn(Self::handle_connection(
                cloned_self.clone(),
                stream.into_split(),
                addr,
            ));
        }
    }

    async fn handle_connection<SRX, STX>(self, stream: (SRX, STX), addr: SocketAddr)
    where
        SRX: AsyncRead + Unpin,
        STX: AsyncWrite + Unpin,
    {
        let (request_sender, request_receiver) = mpsc::channel(REQUEST_CHANNEL_BUFFER);
        let client_id = addr.port().to_string(); // Thats temporary
        let session = Session::new(addr, client_id.clone());
        self.sessions.add(client_id, request_sender).await;
        session.run(stream, request_receiver).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::net::Ipv4Addr;
    use std::net::SocketAddrV4;
    // use tokio::io::{AsyncWriteExt, AsyncReadExt};

    #[tokio::test]
    async fn test_connect() {
        let session_store = session::Store::new();
        let server = Server::new(session_store).await;
        let stream = (Cursor::new(Vec::<u8>::new()), Cursor::new(Vec::<u8>::new()));
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
        tokio::spawn(server.handle_connection(stream.clone(), SocketAddr::V4(addr)));
        // let (rx, tx) = stream;
    }
}
