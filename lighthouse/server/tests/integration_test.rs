use bytes::BytesMut;
use lighthouse_proto::{ClientID, Frame, FrameCodec, ResponseCode};
use lighthouse_server::{connection, tcp};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Encoder};

const SERVER_ADDR: &'static str = "127.0.0.1:23233";

async fn start_server() -> connection::Store {
    let store = connection::Store::new();
    let cloned_store = store.clone();
    tokio::spawn(async move {
        tcp::run(SERVER_ADDR, cloned_store)
            .await
            .expect("failed running server");
    });
    store
}

async fn new_tcpstream() -> TcpStream {
    while let Err(_) = TcpStream::connect(SERVER_ADDR).await {
        println!("Waiting for server to start");
    }
    TcpStream::connect(SERVER_ADDR)
        .await
        .expect("failed connecting")
}

async fn send_connect_frame(stream: &mut TcpStream, client_id: ClientID) -> Frame {
    let mut buf = BytesMut::with_capacity(4096);
    let mut codec = FrameCodec::new();
    let frame = Frame::Connect { client_id };
    codec
        .encode(frame, &mut buf)
        .expect("failed encoding frame");
    stream
        .write_buf(&mut buf)
        .await
        .expect("failed writing to buffer");
    let n = stream
        .read_buf(&mut buf)
        .await
        .expect("failed reading buffer");
    if n == 0 {
        panic!("Received EOF from server");
    }

    codec
        .decode(&mut buf)
        .expect("failed decoding frame")
        .expect("received Ok(None) for decoding")
}

#[tokio::test]
async fn test_connect() {
    let store = start_server().await;
    let mut stream = new_tcpstream().await;
    let client_id = rand::random();
    let response_frame = send_connect_frame(&mut stream, client_id).await;

    let response_code = match response_frame {
        Frame::ConnACK { response_code } => response_code,
        _ => panic!(
            "unexpected frame opcode response: {:?}",
            response_frame.opcode()
        ),
    };

    assert_eq!(response_code, ResponseCode::ConnectionAccepted);
    assert_eq!(store.exists(&client_id).await, true);
}
