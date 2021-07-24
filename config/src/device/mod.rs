use crate::defaults;
use houseflow_types::{DeviceID, DevicePassword};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: Server,

    /// Configuration of the Light device
    pub light: Option<Light>,

    /// Configuration of the Gate device
    pub gate: Option<Gate>,

    /// Configuration of the Garage device
    pub garage: Option<Garage>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    #[serde(default = "defaults::server_hostname", with = "crate::serde_hostname")]
    pub hostname: url::Host,

    #[serde(default)]
    pub use_tls: bool,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            hostname: defaults::server_hostname(),
            use_tls: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Light {
    pub credentials: Credentials,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gate {
    pub credentials: Credentials,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Garage {
    pub credentials: Credentials,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
    /// ID of the device
    pub device_id: DeviceID,

    /// Password of the device in plain-text
    pub device_password: DevicePassword,
}

impl crate::Config for Config {
    fn default_path() -> std::path::PathBuf {
        defaults::config_home().join("device.yaml")
    }

    fn default_yaml() -> String {
        let mut rand = std::iter::repeat_with(|| {
            let random: [u8; 16] = rand::random();
            hex::encode(random)
        });

        format!(
            include_str!("default.yaml"),
            defaults::server_hostname(), // server hostname
            bool::default(),             // use tls
            rand.next().unwrap(),        // light device id
            rand.next().unwrap(),        // light device password
            rand.next().unwrap(),        // gate device id
            rand.next().unwrap(),        // gate device password
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn default_yaml() {
        let config = <Config as crate::Config>::default_yaml();
        let _: Config = serde_yaml::from_str(&config).unwrap();
    }
}
