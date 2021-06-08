use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Clone, Debug, Serialize, Deserialize, StructOpt)]
pub struct ServerConfig {
    /// Key used to sign refresh tokens. Must be secret and should be farily random.
    #[structopt(long = "--refresh-key")]
    pub refresh_key: String,

    /// Key used to sign access tokens. Must be secret and should be farily random.
    #[structopt(long = "--access-key")]
    pub access_key: String,

    #[structopt(flatten)]
    pub auth: AuthServerConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize, StructOpt)]
pub struct AuthServerConfig {
    #[serde(default = "default_auth_port")]
    #[structopt(short = "-p", long = "--port", default_value = "6001")]
    pub port: u16,

    /// Key used to sign access tokens. Must be secret and should be farily random.
    #[structopt(long = "--password-salt")]
    pub password_salt: String,
}

const fn default_auth_port() -> u16 {
    6001
}
