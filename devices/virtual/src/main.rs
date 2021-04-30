use lighthouse_proto::{
    frame::{self, ClientID, Frame},
    FrameCodec, FrameCodecError,
};
use std::convert::TryFrom;
use std::net::ToSocketAddrs;
use structopt::StructOpt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_util::codec::{Decoder, Encoder};

#[derive(StructOpt)]
#[structopt(name = "houseflow-device-virtual")]
struct Opt {
    /// Address of the lighthouse server
    #[structopt(name = "SERVER_ADDRESS")]
    pub address: String,

    /// Client ID
    #[structopt(long, parse(try_from_str = ClientID::try_from))]
    pub client_id: ClientID,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid frame, encoding/decoding failed, error: {0:?}")]
    InvalidFrame(FrameCodecError),

    #[error("Error occured when reading/writing from buffer: {0}")]
    BufferReadWriteError(tokio::io::Error),

    #[error("Error occured when reading/writing from stream: {0}")]
    StreamReadWriteError(tokio::io::Error),

    #[error("Server did not accept connection, reason: {0:?}")]
    UnexpectedConnectResponseCode(frame::connack::ResponseCode),
}

async fn run_connection(mut stream: TcpStream, opt: Opt) -> Result<(), Error> {
    let mut buf = bytes::BytesMut::with_capacity(4096);
    let mut codec = FrameCodec::new();
    let connect_frame = Frame::Connect(frame::connect::Frame {
        client_id: opt.client_id,
    });
    codec
        .encode(connect_frame, &mut buf)
        .map_err(|err| Error::InvalidFrame(err))?;

    stream
        .write_buf(&mut buf)
        .await
        .map_err(|err| Error::StreamReadWriteError(err))?;

    stream
        .read_buf(&mut buf)
        .await
        .map_err(|err| Error::StreamReadWriteError(err))?;

    let frame = codec
        .decode(&mut buf)
        .map_err(|err| Error::InvalidFrame(err))
        .unwrap()
        .unwrap();

    let connack_frame = match frame {
        Frame::ConnACK(frame) => frame,
        _ => panic!(
            "unexpected first frame from server with opcode: {:?}",
            frame.opcode()
        ),
    };

    if connack_frame.response_code != frame::connack::ResponseCode::Accepted {
        return Err(Error::UnexpectedConnectResponseCode(
            connack_frame.response_code,
        ));
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    loggerv::init_with_verbosity(opt.verbose.into()).expect("invalid verbosity level");
    let address = opt.address.to_socket_addrs().unwrap().nth(0).unwrap();
    let stream = TcpStream::connect(address)
        .await
        .expect("failed connecting to TcpStream");
    run_connection(stream, opt)
        .await
        .expect("failed running connection");

    println!("Hello, world!");
}
