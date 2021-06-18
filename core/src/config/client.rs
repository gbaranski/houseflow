use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfig {
    /// Path to keystore, used to store persistant sessions
    /// Default: $XDG_DATA_HOME/houseflow/keystore
    pub keystore_path: PathBuf,

    /// Base URL of the server
    pub base_url: Url,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            keystore_path: xdg::BaseDirectories::with_prefix(clap::crate_name!())
                .unwrap()
                .get_data_home()
                .join("keystore"),
            base_url: super::default_base_url(),
        }
    }
}
