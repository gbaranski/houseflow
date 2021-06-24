use crate::defaults;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(with = "crate::resolve_socket_address")]
    pub address: std::net::SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: defaults::localhost(6379),
        }
    }
}
