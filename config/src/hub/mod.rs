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
    pub accessories: Vec<Accessory>,
    pub providers: Providers,
    pub services: Services,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Server {
    #[serde(default = "defaults::server_websocket_url")]
    pub url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Accessory {
    /// ID of the device
    pub id: device::ID,
    /// Name of the device
    pub name: String,
    /// Type of the accessory, possibly with additional parameters
    #[serde(flatten)]
    pub r#type: AccessoryType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[non_exhaustive]
pub enum AccessoryType {
    XiaomiMijia {
        // TODO: Make it strictly typed
        #[serde(rename = "mac-address")]
        mac_address: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Providers {
    #[serde(default)]
    pub hap: Option<HapProvider>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HapProvider {
    // TODO: Make it strictly typed
    pub pin: String,
    /// Name of the bridge
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Services {
    #[serde(default)]
    pub mijia: Option<MijiaService>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MijiaService {}

impl crate::Config for Config {
    const DEFAULT_TOML: &'static str = include_str!("default.toml");

    const DEFAULT_FILE: &'static str = "hub.toml";

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
    use super::Accessory;
    use super::AccessoryType;
    use super::Config;
    use super::HapProvider;
    use super::MijiaService;
    use super::Providers;
    use super::Server;
    use super::Services;
    use crate::Config as _;
    use houseflow_types::device;
    use url::Url;

    #[test]
    fn test_example() {
        let expected = Config {
            server: Server {
                url: Url::parse("wss://example.com:1234/hello/world").unwrap(),
            },
            accessories: vec![Accessory {
                id: device::ID::parse_str("37c6a8bd-264c-4653-a641-c9b574207be5").unwrap(),
                name: String::from("Thermometer"),
                r#type: AccessoryType::XiaomiMijia {
                    mac_address: "A4:C1:38:EF:77:51".to_string(),
                },
            }],
            providers: Providers {
                hap: Some(HapProvider {
                    pin: "12345678".to_string(),
                    name: "My Home".to_string(),
                }),
            },
            services: Services {
                mijia: Some(MijiaService {}),
            },
        };

        std::env::set_var(
            "SERVER_PORT",
            expected.server.url.port().unwrap().to_string(),
        );
        println!(
            "--------------------\n\n Serialized: \n{}\n\n--------------------",
            toml::to_string(&expected).unwrap()
        );
        let config = Config::parse(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
    }
}
