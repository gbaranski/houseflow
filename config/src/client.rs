use serde::{Deserialize, Serialize};
use crate::defaults;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientConfig {
    /// Path to tokens, used to store persistent sessions
    /// Default: $XDG_DATA_HOME/houseflow/tokens
    pub tokens_path: PathBuf,

    /// Path to devices, used to cache allowed devices
    /// Default: $XDG_DATA_HOME/houseflow/devices
    pub devices_path: PathBuf,

    /// Address of the server
    #[serde(default = "defaults::server_address")]
    #[serde(with = "crate::resolve_socket_address")]
    pub address: std::net::SocketAddr,
}
