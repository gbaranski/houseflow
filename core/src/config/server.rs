use crate::config::defaults;
use serde::{Deserialize, Serialize};
use types::ServerSecrets;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Secret data
    pub secrets: ServerSecrets,

    #[serde(default = "defaults::host")]
    pub host: String,

    #[serde(default = "defaults::port")]
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
