use crate::{Command, ServerCommandState};
use async_trait::async_trait;
use run::RunServerCommand;

mod run;

use clap::Clap;

#[derive(Clap)]
pub enum Service {
    Auth,
    Lighthouse,
    Fulfillment,
}

#[derive(Clap)]
pub enum ServerSubcommand {
    /// Run specific service
    Run (RunServerCommand),
}

#[derive(Clap)]
pub struct ServerCommand {
    #[clap(subcommand)]
    pub subcommand: ServerSubcommand,
}

#[async_trait(?Send)]
impl Command<ServerCommandState> for ServerCommand {
    async fn run(&self, state: ServerCommandState) -> anyhow::Result<()> {
        match &self.subcommand {
            ServerSubcommand::Run(cmd) => cmd.run(state).await
        }
    }
}
