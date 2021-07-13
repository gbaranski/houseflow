use crate::defaults;
use serde::{Deserialize, Serialize};

pub mod google;
pub mod tls;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Server host
    #[serde(default = "defaults::server_hostname", with = "crate::serde_hostname")]
    pub hostname: url::Host,

    /// Path to the SQLite database
    #[serde(default = "defaults::database_path")]
    pub database_path: std::path::PathBuf,

    /// Path to the token store
    #[serde(default = "defaults::token_store_path")]
    pub tokens_path: std::path::PathBuf,

    /// Path to the TLS configuration
    pub tls: Option<tls::Config>,

    /// Secret data
    pub secrets: Secrets,

    /// Configuration of the Google 3rd party service
    pub google: Option<google::Config>,
}

impl Config {
    pub fn default_toml() -> String {
        let mut rand = std::iter::repeat_with(|| {
            let random: [u8; 16] = rand::random();
            hex::encode(random)
        });

        format!(
            include_str!("default.toml"),
            defaults::server_port(),
            defaults::server_hostname(),
            rand.next().unwrap(),
            rand.next().unwrap(),
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
        defaults::config_home().join("server.toml")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Secrets {
    /// Key used to sign refresh tokens. Must be secret and should be farily random.
    pub refresh_key: String,

    /// Key used to sign access tokens. Must be secret and should be farily random.
    pub access_key: String,

    /// Key used to sign authorization codes. Must be secret and should be farily random.
    pub authorization_code_key: String,
}

impl rand::distributions::Distribution<Secrets> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Secrets {
        let mut gen_secret = || {
            let mut bytes = [0; 32];
            rng.fill_bytes(&mut bytes);
            hex::encode(bytes)
        };
        Secrets {
            refresh_key: gen_secret(),
            access_key: gen_secret(),
            authorization_code_key: gen_secret(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn default_toml() {
        let config = Config::default_toml();
        dbg!(&config);
        let _: Config = toml::from_str(&config).unwrap();
    }
}
