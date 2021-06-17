use serde::{Deserialize, Serialize};
use types::{DeviceID, DevicePassword};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// ID of the device
    pub device_id: DeviceID,

    /// Password of the device in plain-text
    pub device_password: DevicePassword,

    /// URL of the Lighthouse service
    /// Default: http://127.0.0.1:6002
    #[serde(default = "default_lighthouse_url")]
    pub lighthouse_url: Url,
}

fn default_lighthouse_url() -> Url {
    Url::parse("http://127.0.0.1:6002").unwrap()
}
