use serde::{Deserialize, Serialize};
use types::{DeviceID, DevicePassword};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// ID of the device
    pub device_id: DeviceID,

    /// Password of the device in plain-text
    pub device_password: DevicePassword,

    /// URL of the server
    #[serde(default = "super::default_base_url")]
    pub base_url: Url,
}
