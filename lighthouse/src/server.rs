use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error)
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}


pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new<T: std::net::ToSocketAddrs>(addr: T) -> Result<Self, Error> {
        let std_tcp_listener = std::net::TcpListener::bind(addr)?;
        Ok(Server {
            listener: TcpListener::from_std(std_tcp_listener)?,
        })
    }

    // Process TCP Stream
    async fn process(mut stream: TcpStream, address: SocketAddr) -> Result<(), Error> {
        log::info!("Client connected from address: {}", address);
        // Consider size of this buffer
        let mut buf = [0; 1024];

        loop {
            let n = stream.read(&mut buf).await?;
            if n == 0 {
                log::info!("Client disconnected");
                return Ok(())
            }

            stream.write_all(b"pong").await?;
        }
    }

    pub async fn run(&self) -> Result<(), Error> {
        loop {
            let (stream, address) = self.listener.accept().await?;
            log::debug!("Accepted connection from: {}", address);

            tokio::spawn(async move {
                log::debug!("starting processing request");
                let resp = Self::process(stream, address).await;
                if let Err(err) = resp {
                    log::error!("Failed processing request: {:?}", err);
                }
            });
        }

    }
}
