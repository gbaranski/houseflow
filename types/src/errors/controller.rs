use crate::accessory;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("accessory not connected")]
    AccessoryNotConnected,
    #[error("accessory error: {0}")]
    AccessoryError(accessory::Error),
    #[error("request timeout")]
    Timeout,
}
