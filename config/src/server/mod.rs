use crate::defaults;
use houseflow_types::{Device, Permission, Room, Structure};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Network configuration
    #[serde(default)]
    pub network: Network,
    /// Secret data
    pub secrets: Secrets,
    /// Path to the TLS configuration
    #[serde(default)]
    pub tls: Option<Tls>,
    /// Configuration of the Google 3rd party service
    #[serde(default)]
    pub google: Option<Google>,
    /// Structures
    #[serde(default)]
    pub structures: Vec<Structure>,
    /// Rooms
    #[serde(default)]
    pub rooms: Vec<Room>,
    /// Devices
    #[serde(default)]
    pub devices: Vec<Device>,
    /// User -> Structure permission
    #[serde(default)]
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

impl Default for Network {
    fn default() -> Self {
        Self {
            hostname: defaults::server_hostname(),
        }
    }
}

use houseflow_types::{DeviceID, RoomID, UserID};

impl Config {
    pub fn get_device(&self, device_id: &DeviceID) -> Option<Device> {
        self.devices
            .iter()
            .find(|device| device.id == *device_id)
            .cloned()
    }

    pub fn get_room(&self, room_id: &RoomID) -> Option<Room> {
        self.rooms.iter().find(|room| room.id == *room_id).cloned()
    }

    pub fn get_permission(&self, device_id: &DeviceID, user_id: &UserID) -> Option<Permission> {
        let device = self
            .devices
            .iter()
            .find(|device| device.id == *device_id)
            .unwrap();
        let room = self
            .rooms
            .iter()
            .find(|room| room.id == device.room_id)
            .unwrap();
        let permission = self.permissions
            .iter()
            .find(|permission| {
                permission.structure_id == room.structure_id && permission.user_id == *user_id
            });

        permission.cloned()
    }

    pub fn get_user_devices(&self, user_id: &UserID) -> Vec<DeviceID> {
        let permissions = self
            .permissions
            .iter()
            .filter(|permission| permission.user_id == *user_id)
            .collect::<Vec<_>>(); // TODO: Maybe remove this collect()
        let rooms = self
            .rooms
            .iter()
            .filter(|room| {
                permissions
                    .iter()
                    .any(|permission| room.structure_id == permission.structure_id)
            })
            .collect::<Vec<_>>(); // TODO: Maybe remove this collect()

        let devices = self
            .devices
            .iter()
            .filter(|device| rooms.iter().any(|room| device.room_id == room.id))
            .map(|device| device.id.to_owned())
            .collect::<Vec<_>>();
        devices
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Google, Network, Secrets, Tls};
    use houseflow_types::{
        Device, DeviceID, DeviceTrait, DeviceType, Permission, Room, RoomID, Structure,
        StructureID, User, UserID,
    };
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

    #[test]
    fn user_permissions() {
        let user_auth = User {
            id: rand::random(),
            username: String::from("gbaranski"),
            email: String::from("root@gbaranski.com"),
            password_hash: String::from("user-auth-password"),
        };
        let user_unauth = User {
            id: rand::random(),
            username: String::from("stanbar"),
            email: String::from("stanbar@gbaranski.com"),
            password_hash: String::from("user-unauth-password"),
        };
        let structure_auth = Structure {
            id: rand::random(),
            name: String::from("Zukago"),
        };
        let structure_unauth = Structure {
            id: rand::random(),
            name: String::from("Gdansk"),
        };
        let room_auth_one = Room {
            id: rand::random(),
            structure_id: structure_auth.id.clone(),
            name: String::from("Bedroom"),
        };
        let room_auth_two = Room {
            id: rand::random(),
            structure_id: structure_auth.id.clone(),
            name: String::from("Garage"),
        };
        let room_unauth_one = Room {
            id: rand::random(),
            structure_id: structure_unauth.id.clone(),
            name: String::from("Bedroom"),
        };
        let room_unauth_two = Room {
            id: rand::random(),
            structure_id: structure_unauth.id.clone(),
            name: String::from("Garage"),
        };
        let device_auth_one = Device {
            id: rand::random(),
            room_id: room_auth_one.id.clone(),
            password_hash: Some(String::from("some-light-password")),
            device_type: DeviceType::Light,
            traits: [DeviceTrait::OnOff].to_vec(),
            name: String::from("Night lamp"),
            will_push_state: false,
            model: String::from("alice"),
            hw_version: Version::new(0, 0, 0),
            sw_version: Version::new(0, 0, 0),
            attributes: Default::default(),
        };
        let device_auth_two = Device {
            id: rand::random(),
            room_id: room_auth_two.id.clone(),
            password_hash: Some(String::from("some-garage-password")),
            device_type: DeviceType::Garage,
            traits: [DeviceTrait::OpenClose].to_vec(),
            name: String::from("garage"),
            will_push_state: false,
            model: String::from("bob"),
            hw_version: Version::new(0, 0, 0),
            sw_version: Version::new(0, 0, 0),
            attributes: Default::default(),
        };
        let device_unauth_one = Device {
            id: rand::random(),
            room_id: room_unauth_one.id.clone(),
            password_hash: Some(String::from("some-light-password")),
            device_type: DeviceType::Light,
            traits: [DeviceTrait::OnOff].to_vec(),
            name: String::from("Night lamp"),
            will_push_state: false,
            model: String::from("alice"),
            hw_version: Version::new(0, 0, 0),
            sw_version: Version::new(0, 0, 0),
            attributes: Default::default(),
        };
        let device_unauth_two = Device {
            id: rand::random(),
            room_id: room_unauth_two.id.clone(),
            password_hash: Some(String::from("some-garage-password")),
            device_type: DeviceType::Garage,
            traits: [DeviceTrait::OpenClose].to_vec(),
            name: String::from("garage"),
            will_push_state: false,
            model: String::from("bob"),
            hw_version: Version::new(0, 0, 0),
            sw_version: Version::new(0, 0, 0),
            attributes: Default::default(),
        };
        let config = Config {
            network: Default::default(),
            secrets: rand::random(),
            tls: Default::default(),
            google: Default::default(),
            structures: [structure_auth.clone(), structure_unauth.clone()].to_vec(),
            rooms: [
                room_auth_one,
                room_auth_two,
                room_unauth_one,
                room_unauth_two,
            ]
            .to_vec(),
            devices: [
                device_auth_one.clone(),
                device_auth_two.clone(),
                device_unauth_one.clone(),
                device_unauth_two.clone(),
            ]
            .to_vec(),
            permissions: [
                Permission {
                    structure_id: structure_auth.id.clone(),
                    user_id: user_auth.id.clone(),
                    is_manager: true,
                },
                Permission {
                    structure_id: structure_unauth.id.clone(),
                    user_id: user_unauth.id.clone(),
                    is_manager: true,
                },
            ]
            .to_vec(),
        };
        let user_auth_devices = config.get_user_devices(&user_auth.id);
        let user_unauth_devices = config.get_user_devices(&user_unauth.id);
        assert_eq!(
            user_auth_devices,
            vec![device_auth_one.id.clone(), device_auth_two.id.clone()]
        );
        assert_eq!(
            user_unauth_devices,
            vec![device_unauth_one.id.clone(), device_unauth_two.id.clone()]
        );

        assert_eq!(
            config.get_permission(&device_auth_one.id, &user_auth.id),
            Some(Permission {
                structure_id: structure_auth.id.clone(),
                user_id: user_auth.id.clone(),
                is_manager: true,
            })
        );
        assert_eq!(
            config.get_permission(&device_auth_two.id, &user_auth.id),
            Some(Permission {
                structure_id: structure_auth.id.clone(),
                user_id: user_auth.id.clone(),
                is_manager: true,
            })
        );
        assert_eq!(
            config.get_permission(&device_unauth_one.id, &user_unauth.id),
            Some(Permission {
                structure_id: structure_unauth.id.clone(),
                user_id: user_unauth.id.clone(),
                is_manager: true,
            })
        );
        assert_eq!(
            config.get_permission(&device_unauth_two.id, &user_unauth.id),
            Some(Permission {
                structure_id: structure_unauth.id.clone(),
                user_id: user_unauth.id.clone(),
                is_manager: true,
            })
        );

        assert_eq!(
            config.get_permission(&device_unauth_one.id, &user_auth.id),
            None
        );
        assert_eq!(
            config.get_permission(&device_unauth_two.id, &user_auth.id),
            None
        );
    }
}
