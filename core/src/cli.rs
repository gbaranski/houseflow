use dialoguer::theme::{ColorfulTheme, Theme};

pub fn get_theme() -> impl Theme {
    ColorfulTheme {
        ..ColorfulTheme::default()
    }
}

use crate::{
    AuthCommand, ConfigCommand, FulfillmentCommand, RunCommand,
};
use clap::Clap;


#[derive(Clap)]
pub enum Subcommand {
    /// Login, register, logout, and refresh your authentication
    Auth(AuthCommand),

    /// Manage the fulfillment service, sync devices, execute command, query state
    Fulfillment(FulfillmentCommand),

    /// Run specific parts of project
    Run(RunCommand),

    /// Manage configuration(s)
    Config(ConfigCommand),
}

use std::path::PathBuf;

#[derive(Clap)]
pub struct CliConfig {
    #[clap(subcommand)]
    pub subcommand: Subcommand,

    /// Server config in TOML format, can be used to pass configuration as argument instead of
    /// editing the config file
    #[clap(long)]
    pub server_config: Option<String>,

    /// Client config in TOML format, can be used to pass configuration as argument instead of
    /// editing the config file
    #[clap(long)]
    pub client_config: Option<String>,

    /// Path to the server config.
    /// Uses `$XDG_CONFIG_HOME/houseflow/server.toml` by default
    #[clap(long)]
    pub server_config_path: Option<PathBuf>,

    /// Path to the server config.
    /// Uses `$XDG_CONFIG_HOME/houseflow/server.toml` by default
    #[clap(long)]
    pub client_config_path: Option<PathBuf>,
}
