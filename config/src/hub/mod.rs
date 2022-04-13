use crate::defaults;
use houseflow_types::accessory;
use houseflow_types::hub;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    pub hub: Hub,
    #[serde(default)]
    pub network: Network,
    #[serde(default)]
    pub accessories: Vec<Accessory>,
    #[serde(default)]
    pub providers: Providers,
    #[serde(default)]
    pub controllers: Controllers,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Hub {
    pub id: hub::ID,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Network {
    #[serde(default = "defaults::listen_address")]
    pub address: std::net::IpAddr,
    #[serde(default = "defaults::hub_port")]
    pub port: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Accessory {
    /// ID of the accessory
    pub id: accessory::ID,
    /// Name of the accessory
    pub name: String,
    /// Name of the room that the accessory is in
    pub room_name: String,
    /// Type of the accessory, possibly with additional parameters
    #[serde(flatten)]
    pub r#type: accessory::Type,
    #[serde(default)]
    // Only some accessories require this
    pub mac_address: Option<String>,
}

impl From<Accessory> for accessory::Accessory {
    fn from(val: Accessory) -> Self {
        accessory::Accessory {
            id: val.id,
            name: val.name,
            room_name: val.room_name,
            r#type: val.r#type,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Providers {
    #[serde(default)]
    pub hive: Option<HiveProvider>,
    #[serde(default)]
    pub mijia: Option<MijiaProvider>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct HiveProvider {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct MijiaProvider {}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Controllers {
    #[serde(default)]
    pub hap: Option<controllers::Hap>,
    #[serde(default)]
    pub lighthouse: Option<controllers::Lighthouse>,
    #[serde(default)]
    pub meta: Option<controllers::Meta>,
}

pub mod controllers {
    use serde::Deserialize;
    use serde::Serialize;
    use url::Url;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case", deny_unknown_fields)]
    pub struct Hap {
        // TODO: Make it strictly typed
        pub pin: String,
        /// Name of the bridge
        pub name: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case", deny_unknown_fields)]
    pub struct Lighthouse {
        pub password: String,
        pub url: Url,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case", deny_unknown_fields)]
    pub struct Meta {}
}

impl crate::Config for Config {
    const DEFAULT_TOML: &'static str = include_str!("default.toml");

    const DEFAULT_FILE: &'static str = "hub.toml";

    fn preprocess(&mut self) -> Result<(), String> {
        Ok(())
    }
}

impl Default for Network {
    fn default() -> Self {
        Self {
            address: defaults::listen_address(),
            port: defaults::hub_port(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use super::*;
    use crate::Config as _;
    use houseflow_types::accessory;
    use url::Url;

    #[test]
    fn test_example() {
        let expected = Config {
            hub: Hub {
                id: hub::ID::parse_str("2adc257a-394c-49bd-ae97-4c5a98b49d84").unwrap(),
            },
            network: Network {
                address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                port: 1234,
            },
            accessories: vec![Accessory {
                id: accessory::ID::parse_str("37c6a8bd-264c-4653-a641-c9b574207be5").unwrap(),
                name: String::from("Thermometer"),
                r#type: accessory::Type::XiaomiMijia(
                    accessory::manufacturers::XiaomiMijia::HygroThermometer,
                ),
                mac_address: Some(String::from("A4:C1:38:EF:77:51")),
                room_name: "Bedroom".to_string(),
            }],
            providers: Providers {
                mijia: Some(MijiaProvider {}),
                hive: None,
            },
            controllers: Controllers {
                hap: Some(controllers::Hap {
                    pin: "12345678".to_string(),
                    name: "Awesome Hub".to_string(),
                }),
                lighthouse: Some(controllers::Lighthouse {
                    url: Url::parse("http://lighthouse").unwrap(),
                    password: String::from("hard-password"),
                }),
                meta: Some(controllers::Meta {}),
            },
        };

        println!(
            "--------------------\n\n Serialized: \n{}\n\n--------------------",
            toml::to_string(&expected).unwrap()
        );
        let config = Config::parse(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
    }
}
