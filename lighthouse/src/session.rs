use crate::Error;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    sync::{mpsc, oneshot, RwLock},
};

pub type RequestSender = mpsc::Sender<Request>;
pub type RequestReceiver = mpsc::Receiver<Request>;

pub type ResponseSender = oneshot::Sender<String>;

// Not yet sure if RwLock is good for that
#[derive(Clone)]
pub struct Store(Arc<RwLock<HashMap<String, RequestSender>>>);

impl Store {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }
    pub async fn add(&self, client_id: String, request_sender: RequestSender) {
        self.0.write().await.insert(client_id, request_sender);
    }

    pub async fn send_to(&self, client_id: &String, request: Request) -> Result<(), Error> {
        self.0
            .read()
            .await
            .get(client_id)
            .ok_or(Error::ClientNotConnected)?
            .send(request)
            .await
            .map_err(|_| Error::Other)
    }
}

pub struct Request {
    data: String,
    response_to: ResponseSender,
}

impl Request {
    pub fn new(data: String, response_to: oneshot::Sender<String>) -> Self {
        Self { data, response_to }
    }
}

pub struct Session {
    client_id: String,
    socket_addr: SocketAddr,
}

type RequestsStore = Arc<RwLock<HashMap<String, oneshot::Sender<String>>>>;

impl Session {
    pub fn new(socket_addr: SocketAddr, client_id: String) -> Self {
        Self {
            socket_addr,
            client_id,
        }
    }

    pub async fn read_requests(
        mut tcp_sender: impl AsyncWrite + Unpin,
        mut request_receiver: RequestReceiver,
        requests_store: RequestsStore,
    ) {
        loop {
            let request = request_receiver
                .recv()
                .await
                .expect("Received empty request");
            log::debug!("Received request, will send");
            requests_store
                .write()
                .await
                .insert(String::from("ABC"), request.response_to);
            tcp_sender
                .write(request.data.as_bytes())
                .await
                .expect("fail sending request");
        }
    }

    pub async fn read_stream(
        mut tcp_receiver: impl AsyncRead + Unpin,
        requests_store: RequestsStore,
    ) {
        let mut buf = [0; 1024];

        loop {
            let _n = match tcp_receiver.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            let text = String::from_utf8(buf.to_vec()).expect("Client sent invalid UTF8 sequence");
            log::debug!("Received text: {}", text);
            let response_to_channel = requests_store
                .write()
                .await
                .remove("ABC")
                .expect("No one was waiting for response");
            response_to_channel
                .send(text)
                .expect("fail sending into oneshot channel");
        }
    }

    pub async fn run(
        self,
        stream: (impl AsyncRead + Unpin, impl AsyncWrite + Unpin),
        request_rx: RequestReceiver,
    ) {
        log::info!(
            "Started session with clientID: {}, address: {}",
            self.client_id,
            self.socket_addr
        );

        let (stream_rx, stream_tx) = stream;
        let requests_store = Arc::new(RwLock::new(HashMap::new()));

        tokio::select! {
            _ = Self::read_stream(stream_rx, requests_store.clone()) => {},
            _ = Self::read_requests(stream_tx, request_rx, requests_store.clone()) => {},
        };
    }
}
