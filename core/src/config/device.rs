use serde::{Deserialize, Serialize};
use types::{DeviceID, DevicePassword};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// ID of the device
    pub device_id: DeviceID,

    /// Password of the device in plain-text
    pub device_password: DevicePassword,

    /// Lighthouse configuration
    pub lighthouse: DeviceLighthouseConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceLighthouseConfig {
    pub host: String,
    pub port: u16,
}

impl Default for DeviceLighthouseConfig {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: lighthouse::server::Config::default().port,
        }
    }
}
