use crate::defaults;
use serde::Deserialize;
use serde::Serialize;

use houseflow_types::permission;
use houseflow_types::structure;
use houseflow_types::user;

use permission::Permission;
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
    /// Mailers configuration
    pub mailers: Mailers,
    #[serde(default)]
    pub controllers: Controllers,
    #[serde(default)]
    pub providers: Providers,
    /// Configuration for login options
    #[serde(default)]
    pub logins: Logins,
    /// Structures
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub structures: Vec<Structure>,
    /// Users
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<User>,
    /// User -> Structure permission
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<Permission>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Network {
    /// Server address
    #[serde(default = "defaults::listen_address")]
    pub address: std::net::IpAddr,
    /// Server port
    #[serde(default = "defaults::server_port")]
    pub port: u16,
    /// Base public URL of server, if different to the listen address and port.
    #[serde(default)]
    pub base_url: Option<Url>,
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
    #[serde(default = "defaults::listen_address")]
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
pub struct Mailers {
    #[serde(default)]
    pub smtp: Option<mailers::Smtp>,
    #[serde(default)]
    pub dummy: Option<mailers::Dummy>,
}

pub mod mailers {
    use serde::Deserialize;
    use serde::Serialize;
    use url::Url;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Smtp {
        pub url: Url,
        pub from: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Dummy {}
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Controllers {
    pub meta: Option<controllers::Meta>,
}

pub mod controllers {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Meta {}
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Providers {
    pub lighthouse: Option<providers::Lighthouse>,
}

pub mod providers {
    use houseflow_types::hub;
    use houseflow_types::structure;
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Lighthouse {
        /// Hubs
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub hubs: Vec<LighthouseHub>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct LighthouseHub {
        pub id: hub::ID,
        pub name: String,
        pub password_hash: String,
        pub structure_id: structure::ID,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Logins {
    /// Configuration for Google login.
    #[serde(default)]
    pub google: Option<GoogleLogin>,
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
        if let Some(mailers::Smtp { url, from: _ }) = &mut self.mailers.smtp {
            if url.port().is_none() {
                let scheme = url.scheme();
                let port = match scheme {
                    "smtp" => defaults::smtp_port(),
                    _ => return Err(format!("unexpected email URL scheme: {}", scheme)),
                };
                url.set_port(Some(port)).unwrap();
            }

            if url.username() == "" {
                tracing::debug!("WARN: username is missing from email URL");
            }

            if url.password().is_none() {
                tracing::debug!("WARN: password is missing from email URL");
            }
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
            address: defaults::listen_address(),
            port: defaults::server_port(),
            base_url: None,
        }
    }
}

impl Config {
    pub fn get_user(&self, user_id: &user::ID) -> Option<&User> {
        self.users.iter().find(|user| user.id == *user_id)
    }

    pub fn get_user_by_email(&self, user_email: &lettre::Address) -> Option<&User> {
        self.users.iter().find(|user| user.email == *user_email)
    }

    pub fn get_structure(&self, id: &structure::ID) -> Option<&Structure> {
        self.structures.iter().find(|structure| structure.id == *id)
    }

    pub fn get_permission(
        &self,
        structure_id: &structure::ID,
        user_id: &user::ID,
    ) -> Option<&Permission> {
        self.permissions.iter().find(|permission| {
            permission.structure_id == *structure_id && permission.user_id == *user_id
        })
    }

    pub fn get_user_structures(&self, user_id: &user::ID) -> Vec<&Structure> {
        self.permissions
            .iter()
            .filter(|permission| permission.user_id == *user_id)
            .map(|permission| {
                self
                    .get_structure(&permission.structure_id)
                    .unwrap_or_else(|| panic!(
                        "dangling permission reference to a structure with id = {}, and user id = {}", permission.structure_id, user_id
                    ))
            }).collect()
    }

    pub fn get_base_url(&self) -> Url {
        self.network.base_url.clone().unwrap_or_else(|| {
            let (scheme, address, port) = if let Some(tls) = &self.tls {
                ("https", &tls.address, &tls.port)
            } else {
                ("http", &self.network.address, &self.network.port)
            };
            Url::parse(&format!("{}://{}:{}", scheme, address, port)).unwrap()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config as _;

    use houseflow_types::hub;
    use std::str::FromStr;
    use url::Url;

    use houseflow_types::permission;
    use houseflow_types::structure;
    use houseflow_types::user;

    use permission::Permission;
    use structure::Structure;
    use user::User;

    #[test]
    fn test_example() {
        let expected = Config {
            network: Network {
                address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
                port: 1234,
                base_url: Some(Url::from_str("http://localhost:1234").unwrap()),
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
            mailers: Mailers {
                smtp: Some(mailers::Smtp {
                    url: Url::from_str(
                        "smtp://gbaranski:haslo123@email.houseflow.gbaranski.com:666",
                    )
                    .unwrap(),
                    from: String::from("houseflow@gbaranski.com"),
                }),
                dummy: Some(mailers::Dummy {}),
            },
            controllers: Controllers {
                meta: Some(controllers::Meta {}),
            },
            providers: Providers {
                lighthouse: Some(providers::Lighthouse {
                    hubs: [providers::LighthouseHub {
                        id: hub::ID::from_str("c3b846ed-74f1-4fd9-90d2-e6c2669dfaa6").unwrap(),
                        name: String::from("Simple Hub"),
                        password_hash: String::from("some-password-hash"),
                        structure_id: structure::ID::from_str("bd7feab5033940e296ed7fcdc700ba65")
                            .unwrap(),
                    }]
                    .to_vec(),
                }),
            },
            logins: Logins {
                google: Some(GoogleLogin {
                    client_id: String::from("google-login-client-id"),
                }),
            },
            structures: [Structure {
                id: structure::ID::from_str("bd7feab5033940e296ed7fcdc700ba65").unwrap(),
                name: String::from("Zukago"),
            }]
            .to_vec(),
            users: [User {
                id: user::ID::from_str("861ccceaa3e349138ce2498768dbfe09").unwrap(),
                username: String::from("gbaranski"),
                email: lettre::Address::from_str("root@gbaranski.com").unwrap(),
                admin: false,
            }]
            .to_vec(),
            permissions: [Permission {
                structure_id: structure::ID::from_str("bd7feab5033940e296ed7fcdc700ba65").unwrap(),
                user_id: user::ID::from_str("861ccceaa3e349138ce2498768dbfe09").unwrap(),
                is_manager: true,
            }]
            .to_vec(),
        };
        std::env::set_var("REFRESH_KEY", &expected.secrets.refresh_key);
        std::env::set_var("ACCESS_KEY", &expected.secrets.access_key);
        std::env::set_var(
            "AUTHORIZATION_CODE_KEY",
            &expected.secrets.authorization_code_key,
        );
        let smtp = expected.mailers.smtp.as_ref().unwrap();
        std::env::set_var("EMAIL_USERNAME", smtp.url.username());
        std::env::set_var("EMAIL_PASSWORD", smtp.url.password().unwrap());
        println!(
            "--------------------\n\n Serialized: \n{}\n\n--------------------",
            toml::to_string(&expected).unwrap()
        );
        let config = Config::parse(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
        crate::Config::validate(&config).unwrap();
    }
}
