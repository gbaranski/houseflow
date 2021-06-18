use serde::{Deserialize, Serialize};
use types::ServerSecrets;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Secret data
    pub secrets: ServerSecrets,

    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    /// Configuration of the auth service
    pub auth: auth::server::Config,

    /// Configuration of the fulfillment service
    pub fulfillment: fulfillment::server::Config,

    /// Configuration of the lighthouse service
    pub lighthouse: lighthouse::server::Config,

    /// Configuration of the PostgreSQL Database
    pub postgres: db::postgres::Config,
}

pub fn default_host() -> String {
    String::from("127.0.0.1")
}

pub fn default_port() -> u16 {
    6001
}
