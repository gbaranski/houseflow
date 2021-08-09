use crate::defaults;
use houseflow_types::{Device, Room, Structure, Permission};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Network configuration
    pub network: Network,
    /// Secret data
    pub secrets: Secrets,
    /// Path to the TLS configuration
    pub tls: Option<Tls>,
    /// Configuration of the Google 3rd party service
    pub google: Option<Google>,
    pub structures: Vec<Structure>,
    pub rooms: Vec<Room>,
    pub devices: Vec<Device>,
    pub permissions: Vec<Permission>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Network {
    /// Server hostname
    #[serde(default = "defaults::server_hostname", with = "crate::serde_hostname")]
    pub hostname: url::Host,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Secrets {
    /// Key used to sign refresh tokens. Must be secret and should be farily random.
    pub refresh_key: String,

    /// Key used to sign access tokens. Must be secret and should be farily random.
    pub access_key: String,

    /// Key used to sign authorization codes. Must be secret and should be farily random.
    pub authorization_code_key: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Tls {
    /// Path to the TLS certificate
    pub certificate: std::path::PathBuf,

    /// Path to the TLS private key
    pub private_key: std::path::PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Google {
    ///  OAuth2 Client ID identifying Google to your service
    pub client_id: String,

    /// OAuth2 Client Secret assigned to the Client ID which identifies Google to you
    pub client_secret: String,

    /// Google Project ID
    pub project_id: String,
}

impl crate::Config for Config {
    const DEFAULT_TOML: &'static str = include_str!("default.toml");

    const DEFAULT_FILE: &'static str = "server.toml";
}

impl rand::distributions::Distribution<Secrets> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Secrets {
        let mut gen_secret = || {
            let mut bytes = [0; 32];
            rng.fill_bytes(&mut bytes);
            hex::encode(bytes)
        };
        Secrets {
            refresh_key: gen_secret(),
            access_key: gen_secret(),
            authorization_code_key: gen_secret(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Google, Network, Secrets, Tls};
    use houseflow_types::{Device, DeviceID, DeviceTrait, DeviceType, Permission, Room, RoomID, Structure, StructureID, UserID};
    use semver::Version;
    use std::str::FromStr;

    #[test]
    fn test_example() {
        let expected = Config {
            network: Network {
                hostname: url::Host::Ipv4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            },
            secrets: Secrets {
                refresh_key: String::from("some-refresh-key"),
                access_key: String::from("some-access-key"),
                authorization_code_key: String::from("some-authorization-code-key"),
            },
            tls: Some(Tls {
                certificate: std::path::PathBuf::from_str("/etc/certificate").unwrap(),
                private_key: std::path::PathBuf::from_str("/etc/private-key").unwrap(),
            }),
            google: Some(Google {
                client_id: String::from("google-client-id"),
                client_secret: String::from("google-client-secret"),
                project_id: String::from("google-project-id"),
            }),
            structures: [Structure {
                id: StructureID::from_str("bd7feab5033940e296ed7fcdc700ba65").unwrap(),
                name: String::from("Zukago"),
            }]
            .to_vec(),
            rooms: [Room {
                id: RoomID::from_str("baafebaa0708441782cf17470dd98392").unwrap(),
                structure_id: StructureID::from_str("bd7feab5033940e296ed7fcdc700ba65").unwrap(),
                name: String::from("Bedroom"),
            }]
            .to_vec(),
            devices: [
                Device {
                    id: DeviceID::from_str("aa9936b052cb4718b77c87961d14c79c").unwrap(),
                    room_id: RoomID::from_str("baafebaa0708441782cf17470dd98392").unwrap(),
                    password_hash: Some(String::from("$argon2i$v=19$m=4096,t=3,p=1$oWC2oDYLWUkx46MehdPiuw$3ibEvJypruiJ1kk4IczUPgbgLKiMOJ6nO+OqiA1Ez6U")),
                    device_type: DeviceType::Light,
                    traits: [DeviceTrait::OnOff].to_vec(),
                    name: String::from("Night Lamp"),
                    will_push_state: true,
                    model: String::from("alice"),
                    hw_version: Version::new(0, 1, 0),
                    sw_version: Version::new(0, 1, 0),
                    attributes: Default::default(),
                }
            ].to_vec(),
            permissions: [
                Permission {
                    structure_id: StructureID::from_str("bd7feab5033940e296ed7fcdc700ba65").unwrap(),
                    user_id: UserID::from_str("861ccceaa3e349138ce2498768dbfe09").unwrap(),
                    is_manager: true,
                }
            ].to_vec(),
        };
        let config = toml::from_str::<Config>(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
    }
}
