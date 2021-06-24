use crate::defaults;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Address of the server
    #[serde(default = "defaults::server_address")]
    #[serde(with = "crate::resolve_socket_address")]
    pub server_address: std::net::SocketAddr,
}

impl Config {
    pub fn default_toml() -> String {
        format!(include_str!("default.toml"), defaults::server_address(),)
    }
}

#[cfg(feature = "fs")]
impl Config {
    pub async fn get(path: std::path::PathBuf) -> Result<Self, std::io::Error> {
        let config = crate::read_file(path).await?;
        Ok(config)
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
