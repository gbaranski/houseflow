use crate::{AuthCommand, ClientConfig, RunCommand, ServerConfig, ConfigCommand};
use clap::Clap;
use async_trait::async_trait;

#[derive(Clap)]
pub enum Subcommand {
    #[clap(flatten)]
    Client(ClientCommand),

    #[clap(flatten)]
    Server(ServerCommand),

    #[clap(flatten)]
    Setup(SetupCommand)
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

#[derive(Clap)]
pub enum SetupCommand {
    /// Manage configurations
    Config(ConfigCommand),
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

#[async_trait(?Send)]
impl crate::SetupCommand for SetupCommand {
    async fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::Config(cmd) => cmd.run().await,
        }
    }
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
