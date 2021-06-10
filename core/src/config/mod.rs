mod cli;
mod client;
mod command;
mod server;
pub use cli::*;
pub use client::*;
pub use command::*;
pub use server::*;

use crate::Target;
use anyhow::Context;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Clone, Debug, Serialize, Deserialize, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum LogLevel {
    /// A level lower than all log levels.
    Off,

    /// Corresponds to the `Error` log level.
    Error,

    /// Corresponds to the `Warn` log level.
    Warn,

    /// Corresponds to the `Info` log level.
    Info,

    /// Corresponds to the `Debug` log level.
    Debug,

    /// Corresponds to the `Trace` log level.
    Trace,
}

impl Into<log::LevelFilter> for LogLevel {
    fn into(self) -> log::LevelFilter {
        use log::LevelFilter;
        match self {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub client: ClientConfig,

    pub server: ServerConfig,
}

use serde::de::DeserializeOwned;
use std::path::PathBuf;

fn read_file<T: DeserializeOwned>(path: &PathBuf) -> anyhow::Result<T> {
    if path.exists() == false {
        let msg = format!("not found at `{}`", path.to_str().unwrap_or("none"));
        return Err(anyhow::Error::msg(msg));
    }

    let content = std::fs::read_to_string(path)?;
    let content = content.as_str();
    let config: T = toml::from_str(content)?;

    Ok(config)
}

pub fn read_files() -> anyhow::Result<Config> {
    fn read_target_config_file<T: DeserializeOwned>(target: Target) -> anyhow::Result<T> {
        read_file(&target.config_path()).with_context(|| format!("{} config", target))
    }

    let server = read_target_config_file(Target::Server)?;
    let client = read_target_config_file(Target::Client)?;

    let config = Config { client, server };

    Ok(config)
}

fn generate_config_string(target: &Target) -> String {
    match target {
        Target::Server => {
            let mut rand = std::iter::repeat_with(|| {
                let random: [u8; 16] = rand::random();
                hex::encode(random)
            });

            let (refresh_key, access_key, password_salt) = (
                rand.next().unwrap(),
                rand.next().unwrap(),
                rand.next().unwrap(),
            );
            format!(
                indoc! {r#"# Houseflow server configuration

                    # Randomly generated keys, keep them safe, don't share with anyone
                    refresh_key = "{}"
                    access_key = "{}"

                    # Configuration of the Auth service
                    [auth]
                    # Randomly generated password salt, keep it safe, don't share with anyone.
                    password_salt = "{}"

                    # Configuration of the Lighthouse service
                    [lighthouse]
                "#},
                refresh_key, access_key, password_salt
            )
        }
        Target::Client => {
            format!("# Houseflow client configuration")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_config_client() {
        let client = generate_config_string(&Target::Client);
        let _: ClientConfig = toml::from_str(&client).unwrap();
    }

    #[test]
    fn test_generate_config_server() {
        let server = generate_config_string(&Target::Server);
        let _: ServerConfig = toml::from_str(&server).unwrap();
    }
}
