use super::{
    channels::{RequestReceiver, ResponseSender},
    error::Error,
    store::Store,
    Request, Response,
};
use bytes::BytesMut;
use lighthouse_proto::{
    frame::{self, Frame},
    FrameCodec,
};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    sync::{mpsc, watch},
};
use tokio_util::codec::{Decoder, Encoder};

/// Starts connection on stream
pub async fn run(
    stream: (impl AsyncRead + Unpin, impl AsyncWrite + Unpin),
    address: SocketAddr,
    store: Store,
) -> Result<(), Error> {
    let mut frame_codec = FrameCodec::new();
    let mut buf = BytesMut::with_capacity(4096);
    let (mut stream_receiver, mut stream_sender) = stream;
    let n = stream_receiver.read_buf(&mut buf).await?;
    if n == 0 {
        return Err(Error::ConnectionResetByPeer);
    }

    let client_id = match frame_codec.decode(&mut buf)? {
        Some(Frame::Connect(frame)) => frame.client_id,

        // First frame should be Connect
        Some(v) => return Err(Error::UnexpectedFrame(v.opcode())),

        // Connection closed by peer
        None => return Ok(()),
    };
    let connack_frame = frame::connack::Frame {
        response_code: frame::connack::ResponseCode::Accepted,
    };
    frame_codec
        .encode(Frame::ConnACK(connack_frame), &mut buf)
        .expect("failed encoding ConnACK Frame");
    stream_sender
        .write_buf(&mut buf)
        .await
        .expect("failed writing connack frame to stream");

    log::info!(
        "Session started with ClientID: `{}` from `{}`",
        client_id,
        address.to_string()
    );

    let (request_sender, request_receiver) = mpsc::channel::<Request>(32);
    let (response_sender, response_receiver) = watch::channel::<Option<Response>>(None);

    store
        .add(client_id, (response_receiver.clone(), request_sender))
        .await;

    log::debug!("Added ClientID: `{}` to store", client_id);

    tokio::select! {
        _ = connection_read_loop(stream_receiver, response_sender, frame_codec.clone()) => {
            log::debug!("Stopped connection read loop");
        },
        _ = connection_write_loop(stream_sender, request_receiver, frame_codec.clone()) => {
            log::debug!("Stopped connection write loop");
        },
    };

    Ok(())
}

async fn connection_read_loop(
    mut stream: impl AsyncRead + Unpin,
    events: ResponseSender,
    frame_codec: FrameCodec,
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

        // TODO: implement this
        // let resp = Response {
        //     data: buf[0..n].to_vec(),
        // };
        // events.send(Some(resp)).expect("failed sending response");
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

