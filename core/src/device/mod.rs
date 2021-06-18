mod run;
pub use run::RunDeviceCommand;

use crate::{Command, DeviceCommandState};
use async_trait::async_trait;
use clap::Clap;

#[derive(Clap)]
pub struct DeviceCommand {
    #[clap(subcommand)]
    pub subcommand: DeviceSubcommand,
}

#[derive(Clap)]
pub enum DeviceSubcommand {
    /// Runs the device
    Run(RunDeviceCommand),
}

#[async_trait(?Send)]
impl Command<DeviceCommandState> for DeviceCommand {
    async fn run(&self, state: DeviceCommandState) -> anyhow::Result<()> {
        match &self.subcommand {
            DeviceSubcommand::Run(cmd) => cmd.run(state).await,
        }
    }
}
