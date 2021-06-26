use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use login::LoginCommand;
use logout::LogoutCommand;
use refresh::RefreshCommand;
use register::RegisterCommand;
use status::StatusCommand;

mod login;
mod logout;
mod refresh;
mod register;
mod status;

use clap::Clap;

#[derive(Clap)]
pub struct AuthCommand {
    #[clap(subcommand)]
    pub subcommand: AuthSubcommand,
}

#[derive(Clap)]
pub enum AuthSubcommand {
    /// Log in to existing Houseflow account
    Login(LoginCommand),

    /// Logout from currently logged account
    Logout(LogoutCommand),

    /// Register a new Houseflow account
    Register(RegisterCommand),

    /// Check current authentication status
    Status(StatusCommand),

    /// Refresh access token
    Refresh(RefreshCommand),
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for AuthCommand {
    async fn run(&self, state: ClientCommandState) -> anyhow::Result<()> {
        match &self.subcommand {
            AuthSubcommand::Login(cmd) => cmd.run(state).await,
            AuthSubcommand::Register(cmd) => cmd.run(state).await,
            AuthSubcommand::Status(cmd) => cmd.run(state).await,
            AuthSubcommand::Logout(cmd) => cmd.run(state).await,
            AuthSubcommand::Refresh(cmd) => cmd.run(state).await,
        }
    }
}
