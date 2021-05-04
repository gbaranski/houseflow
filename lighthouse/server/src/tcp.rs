use std::net::ToSocketAddrs;
use tokio::net::TcpListener;

use super::connection;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}

pub async fn run(addr: impl ToSocketAddrs, store: connection::Store) -> Result<(), Error> {
    let addr = addr
        .to_socket_addrs()
        .expect("Invalid TCP address")
        .nth(0)
        .unwrap();
    log::info!("Starting TCP server at address: {}", addr);
    let listener = TcpListener::bind(addr).await?;
    accept_loop(listener, store).await
}

async fn accept_loop(listener: TcpListener, peers: connection::Store) -> Result<(), Error> {
    loop {
        let (stream, address) = listener.accept().await?;
        let peers = peers.clone();
        tokio::spawn(async move {
            connection::run(stream.into_split(), address, peers)
                .await
                .expect("Connection failed"); // TODO: Change that
        });
    }
}
