use crate::{defaults, postgres, redis};
use serde::{Deserialize, Serialize};

pub mod google;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Host, e.g 127.0.0.1
    #[serde(default = "defaults::server_address")]
    pub address: std::net::SocketAddr,

    /// Secret data
    pub secrets: Secrets,

    /// Configuration of the PostgreSQL Database
    #[serde(default)]
    pub postgres: postgres::Config,

    /// Configuration of the Redis Database
    #[serde(default)]
    pub redis: redis::Config,

    /// Configuration of the Google 3rd party service
    pub google: Option<google::Config>,
}

impl Config {
    pub fn default_toml() -> String {
        let mut rand = std::iter::repeat_with(|| {
            let random: [u8; 16] = rand::random();
            hex::encode(random)
        });

        let pg_defaults = postgres::Config::default();

        format!(
            include_str!("default.toml"),
            defaults::server_port(),
            defaults::server_address(),
            rand.next().unwrap(),
            rand.next().unwrap(),
            rand.next().unwrap(),
            pg_defaults.address,
            pg_defaults.database_name,
            pg_defaults.user,
            pg_defaults.password,
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

    /// Salt used with hashing passwords
    pub password_salt: String,
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
            password_salt: gen_secret(),
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
