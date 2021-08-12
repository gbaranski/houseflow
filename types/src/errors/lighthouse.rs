use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("there is an existing connection with device with the specified ID")]
    AlreadyConnected,
}
