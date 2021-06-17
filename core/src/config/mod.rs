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

fn generate_config_string(target: &Target) -> String {
    match target {
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
            format!(
                indoc! {r#"# Houseflow server configuration

                    # Randomly generated keys, keep them safe, don't share with anyone
                    refresh_key = "{}"
                    access_key = "{}"

                    # Configuration of the Auth service
                    [auth]
                    # Randomly generated password salt, keep it safe, don't share with anyone.
                    password_salt = "{}"

                    # Configuration of the Lighthouse service
                    [lighthouse]

                    # Configuration of the Fulfillment service
                    [fulfillment]
                "#},
                refresh_key, access_key, password_salt
            )
        }
        Target::Client => "# Houseflow client configuration".to_string(),
        Target::Device => "# Houseflow device configuration".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_config_client() {
        let client = generate_config_string(&Target::Client);
        let _: ClientConfig = toml::from_str(&client).unwrap();
    }

    #[test]
    fn test_generate_config_server() {
        let server = generate_config_string(&Target::Server);
        let _: ServerConfig = toml::from_str(&server).unwrap();
    }
}
