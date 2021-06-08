use crate::{ClientCommand, ClientConfig};
use async_trait::async_trait;
use structopt::StructOpt;

use login::LoginCommand;
use register::RegisterCommand;
use status::StatusCommand;

mod login;
mod register;
mod status;

#[derive(StructOpt)]
pub enum AuthCommand {
    /// Log in to existing Houseflow account
    Login(LoginCommand),

    /// Register a new Houseflow account
    Register(RegisterCommand),

    Status(StatusCommand),
}

#[async_trait(?Send)]
impl ClientCommand for AuthCommand {
    async fn run(&self, cfg: ClientConfig) -> anyhow::Result<()> {
        match self {
            Self::Login(cmd) => cmd.run(cfg).await,
            Self::Register(cmd) => cmd.run(cfg).await,
            Self::Status(cmd) => cmd.run(cfg).await,
        }
    }
}
