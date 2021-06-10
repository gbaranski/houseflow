use crate::{AuthCommand, ClientConfig, RunCommand, ServerConfig};
use clap::Clap;
use async_trait::async_trait;

#[derive(Clap)]
pub enum Command {
    Client(ClientCommand),

    Server(ServerCommand),
}

#[derive(Clap)]
pub enum ClientCommand {
    /// Login, register, logout, and refresh your authentication
    Auth(AuthCommand),
}

#[derive(Clap)]
pub enum ServerCommand {
    /// Run server(s)
    Run(RunCommand),
}

#[async_trait(?Send)]
impl crate::ClientCommand for ClientCommand {
    async fn run(&self, cfg: ClientConfig) -> anyhow::Result<()> {
        match self {
            Self::Auth(cmd) => cmd.run(cfg).await,
        }
    }
}

#[async_trait(?Send)]
impl crate::ServerCommand for ServerCommand {
    async fn run(&self, cfg: ServerConfig) -> anyhow::Result<()> {
        match self {
            Self::Run(cmd) => cmd.run(cfg).await,
        }
    }
}


use std::path::PathBuf;

#[derive(Clap)]
pub struct CliConfig {
    #[clap(subcommand)]
    pub command: Command,

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
