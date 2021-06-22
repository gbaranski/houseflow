use crate::config::defaults;
use serde::{Deserialize, Serialize};
use types::ServerSecrets;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host, e.g 127.0.0.1
    #[serde(default = "defaults::host")]
    pub host: String,

    /// Port, e.g 6001
    #[serde(default = "defaults::port")]
    pub port: u16,

    /// Secret data
    pub secrets: ServerSecrets,

    /// Configuration of the auth service
    #[serde(default)]
    pub auth: auth::server::Config,

    /// Configuration of the fulfillment service
    #[serde(default)]
    pub fulfillment: fulfillment::server::Config,

    /// Configuration of the lighthouse service
    #[serde(default)]
    pub lighthouse: lighthouse::server::Config,

    /// Configuration of the PostgreSQL Database
    #[serde(default)]
    pub postgres: db::postgres::Config,
}
