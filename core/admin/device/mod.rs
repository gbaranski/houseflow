mod add;
use add::AddDeviceCommand;

use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct DeviceCommand {
    #[clap(subcommand)]
    subcommand: DeviceSubCommand,
}

#[derive(Clap)]
pub enum DeviceSubCommand {
    Add(AddDeviceCommand)
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for DeviceCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        match self.subcommand {
            DeviceSubCommand::Add(cmd) => cmd.run(state).await,
        }
    }
}
