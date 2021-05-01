use lighthouse_proto::frame::Opcode;
use lighthouse_proto::FrameCodecError;

#[derive(Debug)]
pub enum RequestError {
    /// Client not found when searching in ConnectionsStore
    ClientNotFound,

    /// Response not received in expected time
    Timeout,
}

/// Errors that occurs on the lowest level
#[derive(Debug)]
pub enum Error {
    /// Server did not expect frame of this type
    UnexpectedFrame(Opcode),

    /// Error with decoding/encoding frames
    FrameCodecError(FrameCodecError),

    ConnectionResetByPeer,

    IOError(std::io::Error),
}

impl From<FrameCodecError> for Error {
    fn from(v: FrameCodecError) -> Self {
        Self::FrameCodecError(v)
    }
}

impl From<std::io::Error> for Error {
    fn from(v: std::io::Error) -> Self {
        Self::IOError(v)
    }
}

