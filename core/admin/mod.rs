mod device;
mod room;
mod structure;
mod user_structure;

use device::DeviceCommand;
use room::RoomCommand;
use structure::StructureCommand;
use user_structure::UserStructureCommand;

use crate::{ClientCommandState, Command};
use async_trait::async_trait;
use clap::Clap;

#[derive(Clap)]
pub struct AdminCommand {
    #[clap(subcommand)]
    pub subcommand: AdminSubcommand,
}

#[derive(Clap)]
pub enum AdminSubcommand {
    /// Add/Delete/Update devices
    Device(DeviceCommand),

    /// Add/Delete/Update rooms
    Room(RoomCommand),

    /// Add/Delete/Update structures
    Structure(StructureCommand),

    /// Add/Delete/Update user structures
    UserStructure(UserStructureCommand),
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for AdminCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        match self.subcommand {
            AdminSubcommand::Device(cmd) => cmd.run(state).await,
            AdminSubcommand::Room(cmd) => cmd.run(state).await,
            AdminSubcommand::Structure(cmd) => cmd.run(state).await,
            AdminSubcommand::UserStructure(cmd) => cmd.run(state).await,
        }
    }
}
