use crate::defaults;
use houseflow_types::device;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub server: Server,
    pub credentials: Credentials,
    pub devices: Vec<Device>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Server {
    #[serde(default = "defaults::server_websocket_url")]
    pub url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Credentials {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Device {
    /// ID of the device
    pub id: device::ID,
    /// Password of the device in plain-text
    pub password: device::Password,
    /// Type of the device, possibly with additional parameters
    #[serde(flatten)]
    pub r#type: DeviceType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum DeviceType {
    XiaomiMijia {
        // TODO: Make it strongly typed
        mac_address: String, 
    }
}

impl crate::Config for Config {
    const DEFAULT_TOML: &'static str = include_str!("default.toml");

    const DEFAULT_FILE: &'static str = "device.toml";

    fn preprocess(&mut self) -> Result<(), String> {
        if self.server.url.port().is_none() {
            let scheme = self.server.url.scheme();
            let port = match scheme {
                "ws" => defaults::server_port(),
                "wss" => defaults::server_port_tls(),
                _ => return Err(format!("unexpected scheme: {}", scheme)),
            };
            self.server.url.set_port(Some(port)).unwrap();
        }
        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            url: defaults::server_websocket_url(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use super::Credentials;
    use super::Server;
    use crate::Config as _;
    use url::Url;

    #[test]
    fn test_example() {
        let expected = Config {
            server: Server {
                url: Url::parse("wss://example.com:1234/hello/world").unwrap(),
            },
            credentials: Credentials {},
            devices: vec![],
        };

        println!(
            "--------------------\n\n Serialized: \n{}\n\n--------------------",
            toml::to_string(&expected).unwrap()
        );
        let config = Config::parse(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
    }
}
