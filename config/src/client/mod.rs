use crate::defaults;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub server: Server,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    /// Host of the server
    #[serde(default = "defaults::server_hostname", with = "crate::serde_hostname")]
    pub hostname: url::Host,

    #[serde(default)]
    pub use_tls: bool,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            hostname: defaults::server_hostname(),
            use_tls: Default::default(),
        }
    }
}

impl crate::Config for Config {
    fn default_path() -> std::path::PathBuf {
        defaults::config_home().join("client.yaml")
    }

    fn default_yaml() -> String {
        let defaults = Self::default();
        format!(
            include_str!("default.yaml"),
            defaults.server.hostname, defaults.server.use_tls
        )
    }
}
#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn default_yaml() {
        let config = <Config as crate::Config>::default_yaml();
        let _: Config = serde_yaml::from_str(&config).unwrap();
    }
}
