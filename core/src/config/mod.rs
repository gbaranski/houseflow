mod cli;
mod client;
mod server;
pub use cli::*;
pub use client::*;
pub use server::*;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use strum_macros::EnumString;

#[derive(Clone, Debug, Serialize, Deserialize, StructOpt, EnumString)]
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

#[derive(Clone, Debug, Serialize, Deserialize, StructOpt)]
pub struct Config {
    #[structopt(flatten)]
    pub client: ClientConfig,

    #[structopt(flatten)]
    pub server: ServerConfig,

    #[structopt(short = "-l", long = "--log-level")]
    pub log_level: LogLevel,
}
