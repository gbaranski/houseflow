use crate::defaults;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub server: Server,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Server {
    #[serde(default = "defaults::server_websocket_url")]
    pub url: Url,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            url: defaults::server_http_url(),
        }
    }
}

impl crate::Config for Config {
    const DEFAULT_TOML: &'static str = include_str!("default.toml");

    const DEFAULT_FILE: &'static str = "client.toml";
}

#[cfg(test)]
mod tests {
    use super::Config;
    use super::Server;
    use crate::Config as _;
    use url::Url;

    #[test]
    fn test_example() {
        let expected = Config {
            server: Server {
                url: Url::parse("https://example.com:1234/hello/world").unwrap(),
            },
        };
        std::env::set_var(
            "SERVER_PORT",
            expected.server.url.port().unwrap().to_string(),
        );
        let config = Config::parse(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
    }
}
