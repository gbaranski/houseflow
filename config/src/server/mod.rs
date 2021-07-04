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
    #[serde(with = "serde_token_key")]
    pub refresh_key: houseflow_types::token::Key,

    /// Key used to sign access tokens. Must be secret and should be farily random.
    #[serde(with = "serde_token_key")]
    pub access_key: houseflow_types::token::Key,

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
            refresh_key: gen_secret().into(),
            access_key: gen_secret().into(),
            password_salt: gen_secret(),
        }
    }
}

mod serde_token_key {
    pub fn serialize<S>(key: &houseflow_types::token::Key, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(key))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<houseflow_types::token::Key, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TokenKeyVisitor;
        impl<'de> serde::de::Visitor<'de> for TokenKeyVisitor {
            type Value = houseflow_types::token::Key;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("hex encoded bytes")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                hex::decode(v).map_err(|err| serde::de::Error::custom(err.to_string()))
            }
        }

        deserializer.deserialize_str(TokenKeyVisitor)
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
