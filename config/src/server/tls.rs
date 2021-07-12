use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Path to the TLS certificate
    pub certificate_path: std::path::PathBuf,

    /// Path to the TLS private key
    pub private_key_path: std::path::PathBuf,
}
