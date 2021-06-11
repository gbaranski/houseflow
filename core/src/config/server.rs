use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Key used to sign refresh tokens. Must be secret and should be farily random.
    pub refresh_key: String,

    /// Key used to sign access tokens. Must be secret and should be farily random.
    pub access_key: String,

    /// Configuration of the auth service
    pub auth: AuthServerConfig,

    /// Configuration of the lighthouse service
    pub lighthouse: LighthouseServerConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthServerConfig {
    #[serde(default = "default_auth_port")]
    pub port: u16,

    /// Key used to sign access tokens. Must be secret and should be farily random.
    pub password_salt: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LighthouseServerConfig {
    #[serde(default = "default_lighthouse_port")]
    pub port: u16,
}

const fn default_lighthouse_port() -> u16 {
    6002
}

const fn default_auth_port() -> u16 {
    6001
}
