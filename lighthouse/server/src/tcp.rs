use tokio::net::{TcpListener, ToSocketAddrs};

use super::connection;

pub enum Error {
    IOError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}

pub async fn run(addr: impl ToSocketAddrs, store: connection::Store) -> Result<(), Error> {
    let listener = TcpListener::bind(addr).await?;
    accept_loop(listener, store).await
}

async fn accept_loop(listener: TcpListener, peers: connection::Store) -> Result<(), Error> {
    loop {
        let (stream, address) = listener.accept().await?;
        let peers = peers.clone();
        log::debug!("Accepted");
        log::info!("Accepted");
        tokio::spawn(async move { connection::run(stream.into_split(), address, peers).await });
    }
}
