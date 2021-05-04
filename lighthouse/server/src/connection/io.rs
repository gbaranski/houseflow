use super::{
    channels::{RequestReceiver, ResponseSender},
    error::Error,
    store::Store,
    Request, Response,
};
use bytes::BytesMut;
use lighthouse_proto::{
    frame::{self, ClientID, Frame},
    FrameCodec,
};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    sync::{mpsc, watch},
};
use tokio_util::codec::{Decoder, Encoder};

// Those buffer sizes are approximation
const HANDSHAKE_BUFFER_SIZE: usize = 512;

async fn handshake_read_connect_frame(
    codec: &mut FrameCodec,
    buf: &mut BytesMut,
    rx: &mut (impl AsyncRead + Unpin),
) -> Result<frame::connect::Frame, Error> {
    rx.read_buf(buf).await?;
    codec.decode(buf);
    unimplemented!();
}

async fn handshake(
    codec: &mut FrameCodec,
    stream: &mut (impl AsyncRead + Unpin, impl AsyncWrite + Unpin),
) -> Result<frame::connect::Frame, Error> {
    let mut buf = BytesMut::with_capacity(HANDSHAKE_BUFFER_SIZE);
    let (rx, tx) = stream;
    rx.read_buf(&mut buf).await?;
    let frame = codec.decode(&mut buf)?.unwrap();
    let connect_frame = match frame {
        Frame::Connect(frame) => frame,
        _ => return Err(Error::UnexpectedFrame(frame.opcode())),
    };
    let connack_frame = frame::connack::Frame {
        response_code: frame::connack::ResponseCode::Accepted,
    };
    codec.encode(Frame::ConnACK(connack_frame), &mut buf)?;
    tx.write_buf(&mut buf).await?;
    unimplemented!();
}

async fn read_connect_frame(
    codec: &mut FrameCodec,
    stream_receiver: &mut (impl AsyncRead + Unpin),
    buf: &mut BytesMut,
) -> Result<Option<frame::connect::Frame>, Error> {
    let n = stream_receiver.read_buf(buf).await?;
    if n == 0 {
        return Ok(None);
    }
    let frame = codec.decode(buf)?.unwrap();
    let connect_frame = match frame {
        Frame::Connect(frame) => frame,
        _ => return Err(Error::UnexpectedFrame(frame.opcode())),
    };
    Ok(connect_frame)
}

async fn is_authenticated(client_id: &ClientID) -> bool {
    client_id.to_string() == "576a80b576a94507a72d94c269ab08b9" // TODO: impl real auth
}

/// Starts connection on stream
pub async fn run(
    stream: (impl AsyncRead + Unpin, impl AsyncWrite + Unpin),
    address: SocketAddr,
    store: Store,
) -> Result<(), Error> {
    log::debug!("Accepted TCP connection from: {}", address);
    let mut codec = FrameCodec::new();
    let mut buf = BytesMut::with_capacity(1024);
    let (mut stream_receiver, mut stream_sender) = stream;

    log::debug!("Will read connect frame");
    let connect_frame = read_connect_frame(&mut codec, &mut stream_receiver, &mut buf).await?;
    log::debug!("Checking if authorized");
    let connack_frame = match is_authenticated(&connect_frame.client_id).await {
        true => frame::connack::Frame {
            response_code: frame::connack::ResponseCode::Accepted,
        },
        false => frame::connack::Frame {
            response_code: frame::connack::ResponseCode::Unauthorized,
        },
    };
    log::debug!("Encoding connack frame");
    codec.encode(Frame::ConnACK(connack_frame), &mut buf)?;
    log::debug!("Encoded frame: {:?}", buf);
    stream_sender.write_buf(&mut buf).await?;

    log::info!(
        "Session started with ClientID: `{}` from `{}`",
        connect_frame.client_id,
        address.to_string()
    );

    log::debug!("Will create channels");
    let (request_sender, request_receiver) = mpsc::channel::<Request>(32);
    let (response_sender, response_receiver) = watch::channel::<Option<Response>>(None);
    log::debug!("Will add to store");

    store
        .add(
            connect_frame.client_id,
            (response_receiver.clone(), request_sender),
        )
        .await;

    log::debug!("Added ClientID: `{}` to store", connect_frame.client_id);

    tokio::select! {
        _ = connection_read_loop(stream_receiver, response_sender, codec.clone()) => {
            log::debug!("Stopped connection read loop");
        },
        _ = connection_write_loop(stream_sender, request_receiver, codec.clone()) => {
            log::debug!("Stopped connection write loop");
        },
    };

    Ok(())
}

async fn connection_read_loop(
    mut stream: impl AsyncRead + Unpin,
    events: ResponseSender,
    mut frame_codec: FrameCodec,
) -> Result<(), Error> {
    log::debug!("Started connection read loop");
    let mut buf = BytesMut::with_capacity(4096);

    loop {
        let n = stream
            .read_buf(&mut buf)
            .await
            .expect("fail reading buffer");

        // Connection closed
        if n == 0 {
            return Ok(());
        }

        let frame = frame_codec.decode(&mut buf)?.unwrap();
        match frame {
            Frame::ExecuteResponse(frame) => events
                .send(Some(Response::Execute(frame)))
                .expect("failed sending execute response to events channel"),
            _ => return Err(Error::UnexpectedFrame(frame.opcode())),
        };
    }
}

async fn connection_write_loop(
    mut stream: impl AsyncWrite + Unpin,
    mut events: RequestReceiver,
    mut frame_codec: FrameCodec,
) -> Result<(), Error> {
    log::debug!("Started connection write loop");
    let mut buf = BytesMut::with_capacity(4096);

    loop {
        let request = match events.recv().await {
            Some(v) => v,
            // Channel closed
            None => return Ok(()),
        };
        frame_codec
            .encode(request.into(), &mut buf)
            .expect("Failed encoding frame");

        stream
            .write_buf(&mut buf)
            .await
            .expect("fail writing request data to stream");
    }
}
