mod cli;
mod client;
mod server;
pub use cli::*;
pub use client::*;
pub use server::*;

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

fn read_config_file<T: DeserializeOwned>(path: &PathBuf) -> anyhow::Result<T> {
    if path.exists() == false {
        let msg = format!("not found at `{}`", path.to_str().unwrap_or("none"));
        return Err(anyhow::Error::msg(msg));
    }

    let content = std::fs::read_to_string(path)?;
    let content = content.as_str();
    let config: T = toml::from_str(content)?;

    Ok(config)
}

pub fn read_config_files() -> anyhow::Result<Config> {
    let path = xdg::BaseDirectories::with_prefix(clap::crate_name!())?.get_config_home();
    let server = read_config_file(&path.join("server.toml"))?;
    let client = read_config_file(&path.join("client.toml"))?;

    let config = Config { client, server };

    Ok(config)
}

