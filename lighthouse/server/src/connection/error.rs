use thiserror::Error;
use lighthouse_proto::frame::Opcode;
use lighthouse_proto::FrameCodecError;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Client is not connected")]
    ClientNotConnected,

    #[error("Timeout when sending request")]
    Timeout,
}

/// Errors that occurs on the lowest level
#[derive(Debug, Error)]
pub enum Error {
    #[error("Server received unexpected frame from Client with opcode: {0:?}")]
    UnexpectedFrame(Opcode),

    #[error("Failed encoding/decoding frame, error: {0}")]
    FrameCodecError(FrameCodecError),

    #[error("IO operation failed with error: {source}")]
    IOError{
        #[from]
        source: std::io::Error
    }
}

impl From<FrameCodecError> for Error {
    fn from(v: FrameCodecError) -> Self {
        Self::FrameCodecError(v)
    }
}
