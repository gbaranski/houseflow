mod client;
mod command;
mod device;
mod server;

pub use self::device::*;
pub use client::*;
pub use command::*;
pub use server::*;

use crate::Target;
use indoc::indoc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub client: ClientConfig,

    pub server: ServerConfig,

    pub device: DeviceConfig,
}

use serde::de::DeserializeOwned;
use std::path::Path;

pub async fn read_config_file<T: DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
    if !path.exists() {
        let msg = format!("not found at `{}`", path.to_str().unwrap_or("none"));
        return Err(anyhow::Error::msg(msg));
    }

    let content = tokio::fs::read_to_string(path).await?;
    let content = content.as_str();
    let config: T = toml::from_str(content)?;

    Ok(config)
}

fn generate_config_string(target: &Target) -> anyhow::Result<String> {
    let config = match target {
        Target::Server => {
            let mut rand = std::iter::repeat_with(|| {
                let random: [u8; 16] = rand::random();
                hex::encode(random)
            });

            let (refresh_key, access_key, password_salt) = (
                rand.next().unwrap(),
                rand.next().unwrap(),
                rand.next().unwrap(),
            );

            let postgres_defaults = db::postgres::Config::default();
            format!(
                indoc! {r#"# Houseflow server configuration

                    # Randomly generated secrets, keep them safe, don't share with anyone
                    [secrets]
                    refresh_key = "{}"
                    access_key = "{}"
                    password_salt = "{}"

                    # Change to "0.0.0.0" to allow clients from outside network to connect
                    host = "{}"
                    port = {}

                    # Configuration of the Auth service
                    [auth]

                    # Configuration of the Fulfillment service
                    [fulfillment]

                    # Configuration of the Lighthouse service
                    [lighthouse]

                    # Configuration of the PostgreSQL
                    [postgres]
                    host = "{}"
                    port = {}
                    database_name = "{}"
                    user = "{}"
                    password = "{}"

                "#},
                // Secret
                refresh_key,
                access_key,
                password_salt,
                default_host(),
                default_port(),
                // Postgres
                postgres_defaults.host,
                postgres_defaults.port,
                postgres_defaults.database_name,
                postgres_defaults.user,
                postgres_defaults.password,
            )
        }
        Target::Client => {
            let defaults = ClientConfig::default();
            format!(
                indoc! {r#"# Houseflow client configuration
                keystore_path = "{}"

                base_url = "{}"
            "#},
                defaults.keystore_path.to_str().unwrap(),
                defaults.base_url,
            )
        }
        Target::Device => {
            use rand::Rng;
            use types::DeviceID;

            let device_id: DeviceID = rand::random();
            let mut device_password: [u8; 16] = [0; 16];
            rand::thread_rng().fill(&mut device_password);
            let device_password = hex::encode(device_password);

            format!(
                indoc! {r#"# Houseflow device configuration
                #  Randomly generated credentials
                device_id = "{}"
                device_password = "{}"

                base_url = "{}"
                "#
                },
                device_id,
                device_password,
                default_base_url(),
            )
        }
    };
    Ok(config)
}

use url::Url;

pub fn default_base_url() -> Url {
    Url::parse(&format!("http://{}:{}", default_host(), default_port())).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_config_client() {
        let client = generate_config_string(&Target::Client).unwrap();
        let _: ClientConfig = toml::from_str(&client).unwrap();
    }

    #[test]
    fn generate_config_server() {
        let server = generate_config_string(&Target::Server).unwrap();
        let _: ServerConfig = toml::from_str(&server).unwrap();
    }

    #[test]
    fn generate_config_device() {
        let server = generate_config_string(&Target::Device).unwrap();
        let _: DeviceConfig = toml::from_str(&server).unwrap();
    }
}
