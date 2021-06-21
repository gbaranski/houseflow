use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfig {
    /// Path to tokens, used to store persistent sessions
    /// Default: $XDG_DATA_HOME/houseflow/tokens
    pub tokens_path: PathBuf,

    /// Path to devices, used to cache allowed devices
    /// Default: $XDG_DATA_HOME/houseflow/devices
    pub devices_path: PathBuf,

    /// Base URL of the server
    pub base_url: Url,
}

impl Default for ClientConfig {
    fn default() -> Self {
        let data_home = xdg::BaseDirectories::with_prefix(clap::crate_name!())
            .unwrap()
            .get_data_home();
        Self {
            tokens_path: data_home.join("tokens"),
            devices_path: data_home.join("devices"),
            base_url: crate::config::defaults::base_url(),
        }
    }
}
