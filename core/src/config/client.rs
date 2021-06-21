use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfig {
    /// Path to tokens, used to store persistent sessions
    /// Default: $XDG_DATA_HOME/houseflow/keystore
    pub tokens_path: PathBuf,

    /// Base URL of the server
    pub base_url: Url,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            tokens_path: xdg::BaseDirectories::with_prefix(clap::crate_name!())
                .unwrap()
                .get_data_home()
                .join("tokens"),
            base_url: crate::config::defaults::base_url(),
        }
    }
}
