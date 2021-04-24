use bytes::BytesMut;
use lighthouse_proto::{ClientID, Frame, FrameCodec, FrameCodecError, Opcode, ResponseCode};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    sync::{mpsc, watch, RwLock},
};
use tokio_util::codec::{Decoder, Encoder};

/// 5 seconds timeout for getting response for a request
const REQUEST_TIMEOUT_MILLIS: u64 = 5000;

#[derive(Debug)]
pub enum Error {
    /// Client not found when searching in ConnectionsStore
    ClientNotFound,

    /// Response not received in expected time
    RequestTimeout,
}

/// Errors that occurs on the lowest level
#[derive(Debug)]
pub enum ServerError {
    /// Server did not expect frame of this type
    UnexpectedFrame(Opcode),

    /// Error with decoding/encoding frames
    FrameCodecError(FrameCodecError),

    IOError(std::io::Error),
}

impl From<FrameCodecError> for ServerError {
    fn from(v: FrameCodecError) -> Self {
        Self::FrameCodecError(v)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}

/// Response sent from the Client to Server
#[derive(Debug, Clone)]
pub struct Response {
    data: Vec<u8>,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&self.data).expect("response.data is invalid string")
        )
    }
}

/// Request sent from Server to Client
#[derive(Debug)]
pub struct Request {
    data: Vec<u8>,
}

impl Request {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

/// Thread safe channel which allows sending Requests
pub type RequestSender = mpsc::Sender<Request>;

/// Not thread safe channel which allows reading Requests from RequestSender, this will be used
/// only internally by connection_write_loop()
type RequestReceiver = mpsc::Receiver<Request>;

/// Thread safe channel which allows retrieving Responses
pub type ResponseReceiver = watch::Receiver<Option<Response>>;

/// Not thread safe channel which allows sending Responses to ResponseReceiver, this will be used
/// only internally by connection_read_loop()
type ResponseSender = watch::Sender<Option<Response>>;

/// RequestResponseChannel combines:
/// - RequestSender: used to push new requests to stream
/// - ResponseReceiver: used to retrieve responsese from stream
type RequestResponseChannel = (ResponseReceiver, RequestSender);

/// Store holds thread safe RequestResponseChannels with corresponding ClientIDs
#[derive(Clone)]
pub struct Store {
    inner: Arc<RwLock<HashMap<ClientID, RequestResponseChannel>>>,
}

impl Store {
    /// Creates new store and returns it
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Used to add new Connection to store
    pub async fn add(&self, client_id: ClientID, channel: RequestResponseChannel) {
        self.inner.write().await.insert(client_id, channel);
    }

    /// Sends request over RequestSender channel to connection with specific ClientID
    pub async fn send_request(
        &self,
        client_id: &ClientID,
        request: Request,
    ) -> Result<Response, Error> {
        let (mut rx, tx) = self
            .inner
            .read()
            .await
            .get(client_id)
            .ok_or(Error::ClientNotFound)?
            .clone();

        let timeout = Duration::from_millis(REQUEST_TIMEOUT_MILLIS);

        tx.send(request).await.expect("receiver channel is closed");
        tokio::time::timeout(timeout, rx.changed())
            .await
            .map_err(|_| Error::RequestTimeout)?
            .expect("Sender half has been dropped");
        let response = rx.borrow().clone().expect("Received None response");

        Ok(response)
    }
}

/// Starts connection on stream
pub async fn run(
    stream: (impl AsyncRead + Unpin, impl AsyncWrite + Unpin),
    address: SocketAddr,
    store: Store,
) -> Result<(), ServerError> {
    let mut frame_codec = FrameCodec::new();
    let mut buf = BytesMut::with_capacity(4096);
    let (mut stream_receiver, mut stream_sender) = stream;
    let _ = stream_receiver.read_buf(&mut buf);
    let client_id = match frame_codec.decode(&mut buf)? {
        Some(Frame::Connect { client_id }) => client_id,

        // First frame should be Connect
        Some(v) => return Err(ServerError::UnexpectedFrame(v.opcode())),

        // Connection closed by peer
        None => return Ok(()),
    };
    let connack_frame = Frame::ConnACK {
        response_code: ResponseCode::ConnectionAccepted,
    };
    frame_codec
        .encode(connack_frame, &mut buf)
        .expect("failed encoding ConnACK Frame");
    stream_sender
        .write_buf(&mut buf)
        .await
        .expect("failed writing connack frame to stream");

    log::info!(
        "started with client ID: `{}` from `{}`",
        client_id,
        address.to_string()
    );

    let (request_sender, request_receiver) = mpsc::channel::<Request>(32);
    let (response_sender, response_receiver) = watch::channel::<Option<Response>>(None);

    store
        .add(client_id, (response_receiver.clone(), request_sender))
        .await;

    tokio::select! {
        _ = connection_read_loop(stream_receiver, response_sender) => {},
        _ = connection_write_loop(stream_sender, request_receiver) => {},
    };

    Ok(())
}

async fn connection_read_loop(
    mut stream: impl AsyncRead + Unpin,
    events: ResponseSender,
) -> Result<(), Error> {
    let mut buf = BytesMut::with_capacity(4096);
    loop {
        let n = stream
            .read_buf(&mut buf)
            .await
            .expect("fail reading buffer");
        let resp = Response {
            data: buf[0..n].to_vec(),
        };
        events.send(Some(resp)).expect("failed sending response");
    }
}

async fn connection_write_loop(
    mut stream: impl AsyncWrite + Unpin,
    mut events: RequestReceiver,
) -> Result<(), Error> {
    loop {
        let request = events.recv().await.unwrap(); // TODO: remove this unwrap
        stream
            .write(&request.data)
            .await
            .expect("fail writing request data to stream");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Buf, BufMut};
    use rand::random;
    use std::io::Cursor;
    use std::net::{Ipv4Addr, SocketAddrV4};


    #[tokio::test]
    async fn test_connect() {
        let (mut rx, mut tx) = (Cursor::new(Vec::<u8>::new()), Cursor::new(Vec::<u8>::new()));
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0));
        let store = Store::new();

        run((&mut rx, &mut tx), addr, store)
            .await
            .expect("failed running connection");

        let mut buf = BytesMut::with_capacity(4096);

        let mut codec = FrameCodec::new();
        let connect_frame = Frame::Connect {
            client_id: random(),
        };

        codec
            .encode(connect_frame, &mut buf)
            .expect("failed encoding frame");

        tx.write_buf(&mut buf)
            .await
            .expect("failed writing connect packet to buf");

        while !buf.has_remaining() {
            println!("buf: {:?}", buf);
            rx.read_buf(&mut buf)
                .await
                .expect("failed reading to buffer");
        }

        let response_code = match codec
            .decode(&mut buf)
            .expect("failed decoding using frame codec")
        {
            Some(Frame::ConnACK { response_code }) => response_code,
            Some(frame) => panic!("unexpected packet, opcode: {}", frame.opcode() as u8),
            None => panic!("received EOF"),
        };

        assert_eq!(response_code, ResponseCode::ConnectionAccepted);
    }
}
