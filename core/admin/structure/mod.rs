mod add;
use add::AddStructureCommand;

use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct StructureCommand {
    #[clap(subcommand)]
    subcommand: StructureSubCommand,
}

#[derive(Clap)]
pub enum StructureSubCommand {
    Add(AddStructureCommand)
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for StructureCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        match self.subcommand {
            StructureSubCommand::Add(cmd) => cmd.run(state).await,
        }
    }
}
