use crate::defaults;
use serde::Deserialize;
use serde::Serialize;

use houseflow_types::device;
use houseflow_types::permission;
use houseflow_types::room;
use houseflow_types::structure;
use houseflow_types::user;

use device::Device;
use permission::Permission;
use room::Room;
use structure::Structure;
use url::Url;
use user::User;

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
    /// Configuration of the Email
    pub email: Email,
    /// Configuration of the Google 3rd party client
    #[serde(default)]
    pub google: Option<Google>,
    /// Configuration for Google login
    #[serde(default)]
    pub google_login: Option<GoogleLogin>,
    /// Structures
    #[serde(default)]
    pub structures: Vec<Structure>,
    /// Rooms
    #[serde(default)]
    pub rooms: Vec<Room>,
    /// Devices
    #[serde(default)]
    pub devices: Vec<Device>,
    /// Devices
    #[serde(default)]
    pub users: Vec<User>,
    /// User -> Structure permission
    #[serde(default)]
    pub permissions: Vec<Permission>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Network {
    /// Server address
    #[serde(default = "defaults::server_listen_address")]
    pub address: std::net::IpAddr,
    /// Server port
    #[serde(default = "defaults::server_port")]
    pub port: u16,
    /// Base public URL of server
    #[serde(default = "defaults::base_url")]
    pub base_url: Url,
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
    /// Server address
    #[serde(default = "defaults::server_listen_address")]
    pub address: std::net::IpAddr,
    /// Server port
    #[serde(default = "defaults::server_port_tls")]
    pub port: u16,
    /// Path to the TLS certificate
    pub certificate: std::path::PathBuf,
    /// Path to the TLS private key
    pub private_key: std::path::PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Email {
    /// URL of the email server
    pub url: Url,
    /// E-Mail from which to send emails
    pub from: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Google {
    /// OAuth2 Client ID identifying Google to your service
    pub client_id: String,
    /// OAuth2 Client Secret assigned to the Client ID which identifies Google to you
    pub client_secret: String,
    /// Google Project ID
    pub project_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GoogleLogin {
    /// OAuth2 Client ID identifying your service to Google.
    pub client_id: String,
}

impl crate::Config for Config {
    const DEFAULT_TOML: &'static str = include_str!("default.toml");

    const DEFAULT_FILE: &'static str = "server.toml";

    fn validate(&self) -> Result<(), String> {
        for room in &self.rooms {
            if !self
                .structures
                .iter()
                .any(|structure| structure.id == room.structure_id)
            {
                return Err(format!(
                    "Couldn't find structure with id: {} for room: {}",
                    room.structure_id, room.id
                ));
            }
        }

        for device in &self.devices {
            if !self.rooms.iter().any(|room| room.id == device.room_id) {
                return Err(format!(
                    "Couldn't find room with id: {} for device: {}",
                    device.room_id, device.id
                ));
            }
        }

        for permission in &self.permissions {
            if !self
                .structures
                .iter()
                .any(|structure| structure.id == permission.structure_id)
            {
                return Err(format!(
                    "Couldn't find structure with id: {} for permission: {:?}",
                    permission.structure_id, permission
                ));
            }
            if !self.users.iter().any(|user| user.id == permission.user_id) {
                return Err(format!(
                    "Couldn't find user with id: {} for permission: {:?}",
                    permission.user_id, permission
                ));
            }
        }

        Ok(())
    }

    fn preprocess(&mut self) -> Result<(), String> {
        if self.email.url.port().is_none() {
            let scheme = self.email.url.scheme();
            let port = match scheme {
                "smtp" => defaults::smtp_port(),
                _ => return Err(format!("unexpected email URL scheme: {}", scheme)),
            };
            self.email.url.set_port(Some(port)).unwrap();
        }
        if self.email.url.password().is_none() {
            return Err("missing email URL password".to_string());
        }
        Ok(())
    }
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
            address: defaults::server_listen_address(),
            port: defaults::server_port(),
            base_url: defaults::base_url(),
        }
    }
}

impl Config {
    pub fn get_user(&self, user_id: &user::ID) -> Option<User> {
        self.users.iter().find(|user| user.id == *user_id).cloned()
    }

    pub fn get_user_by_email(&self, user_email: &str) -> Option<User> {
        self.users
            .iter()
            .find(|user| user.email == *user_email)
            .cloned()
    }

    pub fn get_device(&self, device_id: &device::ID) -> Option<Device> {
        self.devices
            .iter()
            .find(|device| device.id == *device_id)
            .cloned()
    }

    pub fn get_room(&self, room_id: &room::ID) -> Option<Room> {
        self.rooms.iter().find(|room| room.id == *room_id).cloned()
    }

    pub fn get_permission(&self, device_id: &device::ID, user_id: &user::ID) -> Option<Permission> {
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
        let permission = self.permissions.iter().find(|permission| {
            permission.structure_id == room.structure_id && permission.user_id == *user_id
        });

        permission.cloned()
    }

    pub fn get_user_devices(&self, user_id: &user::ID) -> Vec<device::ID> {
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
    use super::Config;
    use super::Email;
    use super::Google;
    use super::GoogleLogin;
    use super::Network;
    use super::Secrets;
    use super::Tls;
    use crate::Config as _;

    use semver::Version;
    use std::str::FromStr;
    use url::Url;

    use houseflow_types::device;
    use houseflow_types::permission;
    use houseflow_types::room;
    use houseflow_types::structure;
    use houseflow_types::user;

    use device::Device;
    use permission::Permission;
    use room::Room;
    use structure::Structure;
    use user::User;

    #[test]
    fn test_example() {
        let expected = Config {
            network: Network {
                address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
                port: 1234,
                base_url: Url::from_str("http://localhost:1234").unwrap(),
            },
            secrets: Secrets {
                refresh_key: String::from("some-refresh-key"),
                access_key: String::from("some-access-key"),
                authorization_code_key: String::from("some-authorization-code-key"),
            },
            tls: Some(Tls {
                certificate: std::path::PathBuf::from_str("/etc/certificate").unwrap(),
                private_key: std::path::PathBuf::from_str("/etc/private-key").unwrap(),
                address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(1, 2, 3, 4)),
                port: 4321,
            }),
            email: Email {
                url: Url::from_str("smtp://gbaranski:haslo123@email.houseflow.gbaranski.com:666").unwrap(),
                from: String::from("houseflow@gbaranski.com"),
            },
            google: Some(Google {
                client_id: String::from("google-client-id"),
                client_secret: String::from("google-client-secret"),
                project_id: String::from("google-project-id"),
            }),
            google_login: Some(GoogleLogin {
                client_id: String::from("google-login-client-id"),
            }),
            structures: [Structure {
                id: structure::ID::from_str("bd7feab5033940e296ed7fcdc700ba65").unwrap(),
                name: String::from("Zukago"),
            }]
            .to_vec(),
            rooms: [Room {
                id: room::ID::from_str("baafebaa0708441782cf17470dd98392").unwrap(),
                structure_id: structure::ID::from_str("bd7feab5033940e296ed7fcdc700ba65").unwrap(),
                name: String::from("Bedroom"),
            }]
            .to_vec(),
            devices: [
                Device {
                    id: device::ID::from_str("aa9936b052cb4718b77c87961d14c79c").unwrap(),
                    room_id: room::ID::from_str("baafebaa0708441782cf17470dd98392").unwrap(),
                    password_hash: Some(String::from("$argon2i$v=19$m=4096,t=3,p=1$oWC2oDYLWUkx46MehdPiuw$3ibEvJypruiJ1kk4IczUPgbgLKiMOJ6nO+OqiA1Ez6U")),
                    device_type: device::Type::Light,
                    traits: [device::Trait::OnOff].to_vec(),
                    name: String::from("Night Lamp"),
                    will_push_state: true,
                    model: String::from("alice"),
                    hw_version: Version::new(0, 1, 0),
                    sw_version: Version::new(0, 1, 0),
                    attributes: Default::default(),
                }
            ].to_vec(),
            users: [
                User {
                    id: user::ID::from_str("861ccceaa3e349138ce2498768dbfe09").unwrap(),
                    username: String::from("gbaranski"),
                    email: String::from("root@gbaranski.com"),
                    admin: false,
                }
            ].to_vec(),
            permissions: [
                Permission {
                    structure_id: structure::ID::from_str("bd7feab5033940e296ed7fcdc700ba65").unwrap(),
                    user_id: user::ID::from_str("861ccceaa3e349138ce2498768dbfe09").unwrap(),
                    is_manager: true,
                }
            ].to_vec(),
        };
        std::env::set_var("REFRESH_KEY", &expected.secrets.refresh_key);
        std::env::set_var("ACCESS_KEY", &expected.secrets.access_key);
        std::env::set_var(
            "AUTHORIZATION_CODE_KEY",
            &expected.secrets.authorization_code_key,
        );
        std::env::set_var("EMAIL_USERNAME", expected.email.url.username());
        std::env::set_var("EMAIL_PASSWORD", expected.email.url.password().unwrap());
        println!(
            "--------------------\n\n Serialized: \n{}\n\n--------------------",
            toml::to_string(&expected).unwrap()
        );
        let config = Config::parse(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
        crate::Config::validate(&config).unwrap();
    }

    #[test]
    fn user_permissions() {
        let user_auth = User {
            id: user::ID::new_v4(),
            username: String::from("gbaranski"),
            email: String::from("root@gbaranski.com"),
            admin: false,
        };
        let user_unauth = User {
            id: user::ID::new_v4(),
            username: String::from("stanbar"),
            email: String::from("stanbar@gbaranski.com"),
            admin: false,
        };
        let structure_auth = Structure {
            id: structure::ID::new_v4(),
            name: String::from("Zukago"),
        };
        let structure_unauth = Structure {
            id: structure::ID::new_v4(),
            name: String::from("Gdansk"),
        };
        let room_auth_one = Room {
            id: room::ID::new_v4(),
            structure_id: structure_auth.id.clone(),
            name: String::from("Bedroom"),
        };
        let room_auth_two = Room {
            id: room::ID::new_v4(),
            structure_id: structure_auth.id.clone(),
            name: String::from("Garage"),
        };
        let room_unauth_one = Room {
            id: room::ID::new_v4(),
            structure_id: structure_unauth.id.clone(),
            name: String::from("Bedroom"),
        };
        let room_unauth_two = Room {
            id: room::ID::new_v4(),
            structure_id: structure_unauth.id.clone(),
            name: String::from("Garage"),
        };
        let device_auth_one = Device {
            id: device::ID::new_v4(),
            room_id: room_auth_one.id.clone(),
            password_hash: Some(String::from("some-light-password")),
            device_type: device::Type::Light,
            traits: [device::Trait::OnOff].to_vec(),
            name: String::from("Night lamp"),
            will_push_state: false,
            model: String::from("alice"),
            hw_version: Version::new(0, 0, 0),
            sw_version: Version::new(0, 0, 0),
            attributes: Default::default(),
        };
        let device_auth_two = Device {
            id: device::ID::new_v4(),
            room_id: room_auth_two.id.clone(),
            password_hash: Some(String::from("some-garage-password")),
            device_type: device::Type::Garage,
            traits: [device::Trait::OpenClose].to_vec(),
            name: String::from("garage"),
            will_push_state: false,
            model: String::from("bob"),
            hw_version: Version::new(0, 0, 0),
            sw_version: Version::new(0, 0, 0),
            attributes: Default::default(),
        };
        let device_unauth_one = Device {
            id: device::ID::new_v4(),
            room_id: room_unauth_one.id.clone(),
            password_hash: Some(String::from("some-light-password")),
            device_type: device::Type::Light,
            traits: [device::Trait::OnOff].to_vec(),
            name: String::from("Night lamp"),
            will_push_state: false,
            model: String::from("alice"),
            hw_version: Version::new(0, 0, 0),
            sw_version: Version::new(0, 0, 0),
            attributes: Default::default(),
        };
        let device_unauth_two = Device {
            id: device::ID::new_v4(),
            room_id: room_unauth_two.id.clone(),
            password_hash: Some(String::from("some-garage-password")),
            device_type: device::Type::Garage,
            traits: [device::Trait::OpenClose].to_vec(),
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
            email: Email {
                url: Url::parse("smtp://example.com").unwrap(),
                from: String::new(),
            },
            google: Default::default(),
            google_login: Default::default(),
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
            users: [user_auth.clone(), user_unauth.clone()].to_vec(),
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
