use crate::defaults;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Host, e.g 127.0.0.1
    #[serde(default = "defaults::server_address")]
    pub address: std::net::SocketAddr,

    /// Port, e.g 6001
    #[serde(default = "defaults::server_port")]
    pub port: u16,

    /// Secret data
    pub secrets: Secrets,

    /// Configuration of the PostgreSQL Database
    #[serde(default)]
    pub postgres: Postgres,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Secrets {
    /// Key used to sign refresh tokens. Must be secret and should be farily random.
    pub refresh_key: String,

    /// Key used to sign access tokens. Must be secret and should be farily random.
    pub access_key: String,

    /// Salt used with hashing passwords
    pub password_salt: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Postgres {
    pub address: std::net::SocketAddr,
    pub database_name: String,
    pub user: String,
    pub password: String,
}

impl Default for Postgres {
    fn default() -> Self {
        Self {
            address: defaults::localhost(5432),
            database_name: String::from("houseflow"),
            user: String::from("postgres"),
            password: String::from(""),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Redis {
    pub address: std::net::SocketAddr,
}
