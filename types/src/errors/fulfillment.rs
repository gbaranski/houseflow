use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("hub not connected")]
    HubNotConnected,

    #[error("request timeout")]
    Timeout,
}
