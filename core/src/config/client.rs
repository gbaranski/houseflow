use serde::{Deserialize, Serialize};
use crate::LogLevel;
use std::path::PathBuf;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientConfig {
    #[serde(default)]
    pub log_level: LogLevel,

    /// Path to keystore, used to store persistant sessions
    /// Default: $XDG_DATA_HOME/houseflow/keystore
    #[serde(default = "default_keystore_path")]
    pub keystore_path: PathBuf,

    /// URL of the Auth service
    /// Default: http://127.0.0.1:6001
    #[serde(default = "default_auth_url")]
    pub auth_url: Url,
}

fn default_keystore_path() -> PathBuf {
    xdg::BaseDirectories::with_prefix(clap::crate_name!())
        .unwrap()
        .get_data_home()
        .join("keystore")
}

fn default_auth_url() -> Url {
    Url::parse("http://127.0.0.1:6001").unwrap()
}
