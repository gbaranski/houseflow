mod add;
use add::AddUserStructureCommand;

use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct UserStructureCommand {
    #[clap(subcommand)]
    subcommand: UserStructureSubCommand,
}

#[derive(Clap)]
pub enum UserStructureSubCommand {
    Add(AddUserStructureCommand)
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for UserStructureCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        match self.subcommand {
            UserStructureSubCommand::Add(cmd) => cmd.run(state).await,
        }
    }
}
