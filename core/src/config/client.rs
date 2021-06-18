use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfig {
    /// Path to keystore, used to store persistant sessions
    /// Default: $XDG_DATA_HOME/houseflow/keystore
    pub keystore_path: PathBuf,

    /// Auth service configuration
    pub auth: ClientAuthConfig,

    /// Fulfillment service configuration
    pub fulfillment: ClientFulfillmentConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientAuthConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientFulfillmentConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ClientAuthConfig {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: auth::server::Config::default().port,
        }
    }
}

impl Default for ClientFulfillmentConfig {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: fulfillment::server::Config::default().port,
        }
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            keystore_path: xdg::BaseDirectories::with_prefix(clap::crate_name!())
                .unwrap()
                .get_data_home()
                .join("keystore"),
            auth: ClientAuthConfig::default(),
            fulfillment: ClientFulfillmentConfig::default(),
        }
    }
}
