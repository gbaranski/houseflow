use structopt::StructOpt;
use url::Url;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, StructOpt)]
pub struct ClientConfig {
    /// Path to Keystore, used to store persistant sessions
    /// Default: $XDG_DATA_HOME/houseflow/keystore
    #[structopt(long = "--keystore")]
    pub keystore_path: PathBuf,

    /// URL of the Auth service
    /// Default: http://127.0.0.1:6001
    #[structopt(long = "--auth-url")]
    pub auth_url: Url,

}
