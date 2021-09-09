use crate::defaults;
use houseflow_types::device;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub device_type: device::Type,
    #[serde(default)]
    pub server: Server,
    pub credentials: Credentials,
    #[serde(default)]
    pub traits: HashMap<device::Trait, Trait>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Server {
    #[serde(default = "defaults::server_hostname", with = "crate::serde_hostname")]
    pub hostname: url::Host,
    #[serde(default)]
    pub use_tls: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Trait {
    pub command: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Credentials {
    /// ID of the device
    pub id: device::ID,
    /// Password of the device in plain-text
    pub password: device::Password,
}

impl crate::Config for Config {
    const DEFAULT_TOML: &'static str = include_str!("default.toml");

    const DEFAULT_FILE: &'static str = "device.toml";
}

impl Default for Server {
    fn default() -> Self {
        Self {
            hostname: defaults::server_hostname(),
            use_tls: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use super::Credentials;
    use super::Server;
    use super::Trait;
    use houseflow_types::device;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_example() {
        let mut traits = HashMap::new();
        traits.insert(
            device::Trait::OpenClose,
            Trait {
                command: "echo 1".to_string(),
            },
        );
        let expected = Config {
            device_type: device::Type::Garage,
            server: Server {
                hostname: url::Host::Domain(String::from("example.com")),
                use_tls: true,
            },
            credentials: Credentials {
                id: device::ID::from_str("546c8a4b71f04c78bd338069ad3b174b").unwrap(),
                password: String::from("garage-password"),
            },
            traits,
        };
        println!(
            "--------------------\n\n Serialized: \n{}\n\n--------------------",
            toml::to_string(&expected).unwrap()
        );
        let config = toml::from_str::<Config>(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
    }
}
