use bytes::BytesMut;
use lazy_static::lazy_static;
use lighthouse_proto::{
    frame::{self, ClientID, Frame},
    FrameCodec,
};
use lighthouse_server::{connection, tcp};
use rand::random;
use std::sync::Once;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Decoder, Encoder};

const SERVER_ADDR: &'static str = "127.0.0.1:9997";

lazy_static! {
    static ref STORE: connection::Store = connection::Store::new();
}

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {
        tokio::spawn(async move {
            tcp::run(SERVER_ADDR, STORE.clone())
                .await
                .expect("failed running server");
        });
    })
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
    let frame = frame::connect::Frame { client_id };
    codec
        .encode(Frame::Connect(frame), &mut buf)
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
    setup();
    let mut stream = new_tcpstream().await;
    let client_id = random();
    let response_frame = send_connect_frame(&mut stream, client_id).await;

    let response_code = match response_frame {
        Frame::ConnACK(frame) => frame.response_code,
        _ => panic!(
            "unexpected frame opcode response: {:?}",
            response_frame.opcode()
        ),
    };

    assert_eq!(response_code, frame::connack::ResponseCode::Accepted);
    assert_eq!(STORE.exists(&client_id).await, true);
}

#[tokio::test]
async fn test_publish() {
    setup();
    let mut codec = FrameCodec::new();
    let mut stream = new_tcpstream().await;
    let client_id = random();
    let response_frame = send_connect_frame(&mut stream, client_id).await;

    let response_code = match response_frame {
        Frame::ConnACK(frame) => frame.response_code,
        _ => panic!(
            "unexpected frame opcode response: {:?}",
            response_frame.opcode()
        ),
    };

    assert_eq!(response_code, frame::connack::ResponseCode::Accepted);
    assert_eq!(STORE.exists(&client_id).await, true);

    let mut buf = BytesMut::with_capacity(4096);
    let params = r#"
    {
        "on": true,
        "online": true,
        "openPercent": 20
    }
    "#;
    let execute_frame = frame::execute::Frame {
        id: random(),
        command: random(),
        params: serde_json::from_str(&params).unwrap(),
    };
    FrameCodec::new()
        .encode(Frame::Execute(execute_frame), &mut buf)
        .expect("Fail encoding execute_frame");

    // TODO: Test sending execute frame
    
    // let resp = STORE
    //     .send_request(&client_id, connection::Request::new(buf.to_vec()))
    //     .await
    //     .expect("failed sending request");
    // let mut buf = BytesMut::from(resp.data.as_slice());

    // let execute_response_frame = match codec
    //     .decode(&mut buf)
    //     .expect("Failed decoding execute response")
    //     .expect("Received None")
    // {
    //     Frame::ExecuteResponse { dsad } => v,
    //     _ => panic!("Unexpected frame received"),
    // };
}
