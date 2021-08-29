use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("missing header")]
    MissingHeader,

    #[error("missing payload")]
    MissingPayload,

    #[error("missing signature")]
    MissingSignature,

    #[error("invalid json: {0}")]
    InvalidJSON(String),

    #[error("invalid encoding: `{0}`")]
    InvalidEncoding(String),

    #[error("invalid signature")]
    InvalidSignature,

    #[error("token is expired since {seconds} seconds")]
    Expired { seconds: u64 },
}
