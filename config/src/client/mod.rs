use crate::defaults;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Host of the server
    #[serde(default = "defaults::server_hostname", with = "crate::serde_hostname")]
    pub server_hostname: url::Host,

    #[serde(default)]
    pub use_tls: bool,
}

impl Config {
    pub fn default_toml() -> String {
        format!(include_str!("default.toml"), defaults::server_hostname(),)
    }
}

#[cfg(feature = "fs")]
impl Config {
    pub async fn get(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
        crate::read_file(path, Self::default_toml).await
    }

    pub fn default_path() -> std::path::PathBuf {
        defaults::config_home().join("client.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn default_toml() {
        let config = Config::default_toml();
        let _: Config = toml::from_str(&config).unwrap();
    }
}
