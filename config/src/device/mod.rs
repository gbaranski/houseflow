use crate::defaults;
use houseflow_types::{DeviceID, DevicePassword};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub server: Server,

    /// Configuration of the Garage device
    pub garage: Option<Garage>,

    /// Configuration of the Gate device
    pub gate: Option<Gate>,

    /// Configuration of the Light device
    pub light: Option<Light>,

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
pub struct Garage {
    pub credentials: Credentials,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Gate {
    pub credentials: Credentials,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Light {
    pub credentials: Credentials,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Credentials {
    /// ID of the device
    pub device_id: DeviceID,

    /// Password of the device in plain-text
    pub device_password: DevicePassword,
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
    use super::{Config, Credentials, Garage, Gate, Light, Server};
    use houseflow_types::DeviceID;
    use std::str::FromStr;

    #[test]
    fn test_example() {
        let expected = Config {
            server: Server {
                hostname: url::Host::Domain(String::from("example.com")),
                use_tls: true,
            },
            garage: Some(Garage {
                credentials: Credentials {
                    device_id: DeviceID::from_str("546c8a4b71f04c78bd338069ad3b174b").unwrap(),
                    device_password: String::from("garage-password"),
                },
            }),
            gate: Some(Gate {
                credentials: Credentials {
                    device_id: DeviceID::from_str("efeec97b9835430cb719e6f62690a72d").unwrap(),
                    device_password: String::from("gate-password"),
                },
            }),
            light: Some(Light {
                credentials: Credentials {
                    device_id: DeviceID::from_str("86dda092906147938483618eb513c92c").unwrap(),
                    device_password: String::from("light-password"),
                },
            }),
        };
        let config = toml::from_str::<Config>(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
    }
}
