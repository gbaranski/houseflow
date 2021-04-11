use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::{mpsc, oneshot, RwLock},
};

pub type RequestSender = mpsc::Sender<Request>;
pub type RequestReceiver = mpsc::Receiver<Request>;

pub type ResponseSender = oneshot::Sender<String>;

pub type SessionStore = Arc<RwLock<HashMap<String, RequestSender>>>;

pub struct Request {
    data: String,
    response_to: ResponseSender,
}

impl Request {
    pub fn new(data: String, response_to: oneshot::Sender<String>) -> Self {
        Self {
            data,
            response_to,
        }
    }

}

pub struct Session {
    client_id: String,
    socket_addr: SocketAddr,
    request_receiver: RequestReceiver,
    stream: TcpStream,
}

type RequestsStore = Arc<RwLock<HashMap<String, oneshot::Sender<String>>>>;

impl Session {
    pub fn new(request_receiver: RequestReceiver, stream: TcpStream, socket_addr: SocketAddr, client_id: String) -> Self {
        Self {
            request_receiver,
            stream,
            socket_addr,
            client_id,
        }
    }

    pub async fn read_requests(
        mut request_receiver: RequestReceiver,
        mut tcp_sender: OwnedWriteHalf,
        requests_store: RequestsStore,
    ) {
        loop {
            let request = request_receiver
                .recv()
                .await
                .expect("Received empty request");
            log::debug!("Received request, will send");
            requests_store.write().await.insert(String::from("ABC"), request.response_to);
            tcp_sender.write(request.data.as_bytes()).await.expect("fail sending request");
        }
    }

    pub async fn read_stream(mut tcp_receiver: OwnedReadHalf, requests_store: RequestsStore) {
        let mut buf = [0; 1024];

        loop {
            let n = match tcp_receiver.read(&mut buf).await {
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
            response_to_channel.send(text).expect("fail sending into oneshot channel");
        }
    }

    pub async fn run(self) {
        let (tcp_receiver, tcp_sender) = self.stream.into_split();
        let requests_store = Arc::new(RwLock::new(HashMap::new()));

        tokio::select! {
            _ = Self::read_stream(tcp_receiver, requests_store.clone()) => {
            },
            _ = Self::read_requests(self.request_receiver, tcp_sender, requests_store.clone()) => {
            },
        };
    }
}
