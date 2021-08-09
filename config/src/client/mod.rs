use crate::defaults;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub server: Server,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
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
    const DEFAULT_TOML: &'static str = include_str!("default.toml");

    const DEFAULT_FILE: &'static str = "client.toml";
}

#[cfg(test)]
mod tests {
    use super::{Config, Server};

    #[test]
    fn test_example() {
        let expected = Config {
            server: Server {
                hostname: url::Host::Domain(String::from("example.com")),
                use_tls: true,
            },
        };
        let config = toml::from_str::<Config>(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
    }
}
