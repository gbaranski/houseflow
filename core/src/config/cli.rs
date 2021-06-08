use crate::{AuthCommand, ClientConfig, Config, RunCommand, ServerConfig};
use async_trait::async_trait;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Command {
    #[structopt(flatten)]
    Client(ClientCommand),

    #[structopt(flatten)]
    Server(ServerCommand),
}

#[derive(StructOpt)]
pub enum ClientCommand {
    /// Login, register, logout, and refresh your authentication
    Auth(AuthCommand),
}

#[derive(StructOpt)]
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

#[derive(StructOpt)]
pub struct CliConfig {
    #[structopt(subcommand)]
    pub command: Command,

    #[structopt(flatten)]
    pub config: Config,
}
