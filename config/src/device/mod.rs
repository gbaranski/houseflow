use crate::defaults;
use houseflow_types::DeviceID;
use houseflow_types::DevicePassword;
use houseflow_types::DeviceTrait;
use houseflow_types::DeviceType;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub device_type: DeviceType,
    #[serde(default)]
    pub server: Server,
    pub credentials: Credentials,
    pub traits: HashMap<DeviceTrait, Trait>,
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
    pub id: DeviceID,
    /// Password of the device in plain-text
    pub password: DevicePassword,
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
    use houseflow_types::DeviceID;
    use houseflow_types::DeviceTrait;
    use houseflow_types::DeviceType;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_example() {
        let mut traits = HashMap::new();
        traits.insert(
            DeviceTrait::OpenClose,
            Trait {
                command: "echo 1".to_string(),
            },
        );
        let expected = Config {
            device_type: DeviceType::Garage,
            server: Server {
                hostname: url::Host::Domain(String::from("example.com")),
                use_tls: true,
            },
            credentials: Credentials {
                id: DeviceID::from_str("546c8a4b71f04c78bd338069ad3b174b").unwrap(),
                password: String::from("garage-password"),
            },
            traits,
        };
        println!("--------------------\n\n Serialized: \n{}\n\n--------------------", toml::to_string(&expected).unwrap());
        let config = toml::from_str::<Config>(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
    }
}
