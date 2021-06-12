use crate::{ClientCommand, ClientConfig};
use async_trait::async_trait;

use login::LoginCommand;
use register::RegisterCommand;
use status::StatusCommand;

mod login;
mod register;
mod status;

use clap::Clap;

#[derive(Clap)]
pub struct AuthCommand {
    #[clap(subcommand)]
    pub subcommand: AuthSubcommand
}

#[derive(Clap)]
pub enum AuthSubcommand {
    /// Log in to existing Houseflow account
    Login(LoginCommand),

    /// Register a new Houseflow account
    Register(RegisterCommand),

    /// Check current authentication status
    Status(StatusCommand),
}

#[async_trait(?Send)]
impl ClientCommand for AuthCommand {
    async fn run(&self, cfg: ClientConfig) -> anyhow::Result<()> {
        match &self.subcommand {
            AuthSubcommand::Login(cmd) => cmd.run(cfg).await,
            AuthSubcommand::Register(cmd) => cmd.run(cfg).await,
            AuthSubcommand::Status(cmd) => cmd.run(cfg).await,
        }
    }
}
