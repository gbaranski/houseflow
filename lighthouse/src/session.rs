use super::*;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp;
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::{mpsc, oneshot, Mutex, RwLock};
use uuid::Uuid;

/// Thread safe wrapper for session hashmap
pub type SessionStore = Arc<Mutex<HashMap<Uuid, Arc<Mutex<Session>>>>>;

pub struct ChannelRequest {
    request: Option<Request>,
    respond_to: Option<oneshot::Sender<Response>>,
}

#[derive(Clone)]
pub struct Session {
    pub client_id: Uuid,
    addr: SocketAddr,

    rx: Arc<Mutex<OwnedReadHalf>>,
    tx: Arc<Mutex<OwnedWriteHalf>>,
    request_store: Arc<Mutex<HashMap<Uuid, ChannelRequest>>>,
    session_store: SessionStore,
}

impl Session {
    /// Waits for first CONNECT packet, if it doesn't return anything then it returns Error,
    /// otherwise makes new session
    pub async fn new(addr: SocketAddr, session_store: SessionStore, stream: TcpStream) -> Self {
        let client_id = Uuid::new_v4();
        let (rx, tx) = stream.into_split();

        Self {
            // receiver: mpsc::channel(),
            client_id,
            addr,
            session_store,
            rx: Arc::new(Mutex::new(rx)),
            tx: Arc::new(Mutex::new(tx)),
            request_store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn send_request(&self, request: Request) -> Result<Response, Error> {
        let request_id = Uuid::new_v4();
        log::debug!("Sending request with ID: {}", request_id);
        let (resp_tx, resp_rx) = oneshot::channel::<Response>();
        let channel_request = ChannelRequest {
            request: request.into(),
            respond_to: Some(resp_tx),
        };
        self.tx.lock().await.write(b"hello world").await.unwrap();
        self.request_store.lock().await.insert(request_id, channel_request);

        resp_rx.await.map_err(|err| Error::IOError(err.to_string()))
    }

    pub async fn send_execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse, Error> {
        self.send_request(request.into()).await.map(|e| e.into())
    }

    pub async fn send_query(&self, request: QueryRequest) -> Result<QueryResponse, Error> {
        self.send_request(request.into()).await.map(|e| e.into())
    }

    async fn read_stream_loop(&self) {
        let mut buf: [u8; 1024] = [0; 1024];
        loop {
            let n = match self.rx.lock().await.read(&mut buf).await {
                // Connection closed
                Ok(n) if n == 0 => {
                    return;
                }
                Ok(n) => n,
                Err(e) => {
                    log::error!("failed reading from socket, err: {}", e);
                    return;
                }
            };

            let id = Uuid::parse_str(std::str::from_utf8(&buf).unwrap()).unwrap();

            let mut request_store = self.request_store.lock().await;
            let request = request_store.get_mut(&id).unwrap();
            let resp = Response::Execute( ExecuteResponse {
                status: ResponseStatus::Success,
                states: HashMap::new(),
                error: None,
            } );
            request.respond_to.take().unwrap().send(resp).ok().unwrap(); // temp
        }
    }

    async fn read_loop(self) {
        self.read_stream_loop().await;

        log::info!(
            "Connection closed with client ID: {}, address: {}",
            self.client_id,
            self.addr
        );
    }

    pub async fn run(self) {
        let client_id = self.client_id.clone();
        let self_arc = Arc::new(Mutex::new(self));
        self_arc
            .lock()
            .await
            .session_store
            .lock()
            .await
            .insert(client_id, self_arc.clone());
        Self::read_loop(self_arc.lock().await.clone()).await;
        self_arc
            .lock()
            .await
            .session_store
            .lock()
            .await
            .remove(&client_id);
    }
}
