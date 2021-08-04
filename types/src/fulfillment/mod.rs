pub mod execute;
pub mod query;
pub mod sync;

pub mod ghome;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum FulfillmentError {
    #[error("no device permission")]
    NoDevicePermission,

    #[error("Device is not connected")]
    DeviceNotConnected,

    #[error("error with device communication: {0}")]
    DeviceCommunicationError(#[from] crate::lighthouse::DeviceCommunicationError),
}
