use crate::defaults;
use houseflow_types::accessory;
use houseflow_types::accessory::services::ServiceName;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub hub: Hub,
    pub credentials: Credentials,
    #[serde(default)]
    pub services: Services,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Hub {
    #[serde(default = "defaults::hub_websocket_url")]
    pub url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Credentials {
    /// ID of the accessory
    pub id: accessory::ID,
    /// Password of the accessory in plain-text
    pub password: accessory::Password,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Services {
    pub temperature_sensor: Option<services::TemperatureSensor>,
}

impl Services {
    pub fn names(&self) -> Vec<ServiceName> {
        let Self { temperature_sensor } = &self;
        let mut names = vec![];
        if temperature_sensor.is_some() {
            names.push(ServiceName::TemperatureSensor);
        }
        names
    }
}

pub mod services {
    use super::characteristics;
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case", deny_unknown_fields)]
    pub struct TemperatureSensor {
        pub current_temperature: characteristics::CurrentTemperature,
    }
}

pub mod characteristics {
    use crate::Command;
    use serde::Deserialize;
    use serde::Serialize;
    use serde_with::DurationSeconds;
    use std::time::Duration;

    #[serde_with::serde_as]
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case", deny_unknown_fields)]
    pub struct CurrentTemperature {
        pub command: Command,
        #[serde_as(as = "DurationSeconds<u64>")]
        pub interval: Duration,
    }
}

impl crate::Config for Config {
    const DEFAULT_TOML: &'static str = include_str!("default.toml");

    const DEFAULT_FILE: &'static str = "accessory.toml";

    fn preprocess(&mut self) -> Result<(), String> {
        if self.hub.url.port().is_none() {
            let scheme = self.hub.url.scheme();
            let port = match scheme {
                "ws" => defaults::server_port(),
                "wss" => defaults::server_port_tls(),
                _ => return Err(format!("unexpected scheme: {}", scheme)),
            };
            self.hub.url.set_port(Some(port)).unwrap();
        }
        Ok(())
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self {
            url: defaults::hub_websocket_url(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config as _;
    use houseflow_types::accessory;
    use url::Url;

    #[test]
    fn test_example() {
        let expected = Config {
            credentials: Credentials {
                id: accessory::ID::parse_str("345469C1-6C6F-461A-AB60-E21578D5A608").unwrap(),
                password: accessory::Password::from("some-password"),
            },
            hub: Hub {
                url: Url::parse("wss://example.com:1234/hello/world").unwrap(),
            },
            services: Services::default(),
        };

        std::env::set_var("HUB_PORT", expected.hub.url.port().unwrap().to_string());
        println!(
            "--------------------\n\n Serialized: \n{}\n\n--------------------",
            toml::to_string(&expected).unwrap()
        );
        let config = Config::parse(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
    }
}
