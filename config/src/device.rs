use serde::{Deserialize, Serialize};
use types::{DeviceID, DevicePassword};
use crate::defaults;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// ID of the device
    pub device_id: DeviceID,

    /// Password of the device in plain-text
    pub device_password: DevicePassword,

    /// Address of the server
    #[serde(default = "defaults::server_address")]
    #[serde(with = "crate::resolve_socket_address")]
    pub address: std::net::SocketAddr,
}
