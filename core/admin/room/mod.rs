mod add;
use add::AddRoomCommand;

use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct RoomCommand {
    #[clap(subcommand)]
    subcommand: RoomSubCommand,
}

#[derive(Clap)]
pub enum RoomSubCommand {
    Add(AddRoomCommand)
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for RoomCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        match self.subcommand {
            RoomSubCommand::Add(cmd) => cmd.run(state).await,
        }
    }
}
