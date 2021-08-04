pub mod proto;

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum DeviceCommunicationError {
    #[error("Timeout when sending request to device")]
    Timeout,

    #[error("invalid JSON")]
    InvalidJSON(String),
}

impl From<serde_json::Error> for DeviceCommunicationError {
    fn from(val: serde_json::Error) -> Self {
        Self::InvalidJSON(val.to_string())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum ConnectError {
    #[error("invalid authorization header: {0}")]
    InvalidAuthorizationHeader(String),

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("there is an existing connection with device with the specified ID")]
    AlreadyConnected,
}
