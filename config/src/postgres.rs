use crate::defaults;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(with = "crate::resolve_socket_address")]
    pub address: std::net::SocketAddr,

    pub database_name: String,

    pub user: String,

    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: defaults::localhost(5432),
            database_name: String::from("houseflow"),
            user: String::from("postgres"),
            password: String::from(""),
        }
    }
}
