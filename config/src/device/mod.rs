use crate::defaults;
use serde::{Deserialize, Serialize};
use types::{DeviceID, DevicePassword};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// ID of the device
    pub device_id: DeviceID,

    /// Password of the device in plain-text
    pub device_password: DevicePassword,

    /// Address of the server
    #[serde(default = "defaults::server_address")]
    #[serde(with = "crate::resolve_socket_address")]
    pub server_address: std::net::SocketAddr,
}

impl Config {
    pub fn default_toml() -> String {
        let mut rand = std::iter::repeat_with(|| {
            let random: [u8; 16] = rand::random();
            hex::encode(random)
        });

        format!(
            include_str!("default.toml"),
            rand.next().unwrap(),
            rand.next().unwrap(),
            defaults::server_address(),
        )
    }
}

#[cfg(feature = "fs")]
impl Config {
    pub async fn get(path: std::path::PathBuf) -> Result<Self, std::io::Error> {
        let config = crate::read_file(path).await?;
        Ok(config)
    }

    pub fn default_path() -> std::path::PathBuf {
        defaults::config_home().join("device.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn default_toml() {
        let config = Config::default_toml();
        let _: Config = toml::from_str(&config).unwrap();
    }
}
