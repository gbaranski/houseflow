use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub enum DeviceError {
    #[error("Device is not connected")]
    NotConnected,

    #[error("Timeout when sending request to device")]
    Timeout,
}
