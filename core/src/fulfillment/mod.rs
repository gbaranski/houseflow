mod sync;

use crate::{ClientCommand, ClientCommandState};
use async_trait::async_trait;

use sync::SyncCommand;

use clap::Clap;

#[derive(Clap)]
pub struct FulfillmentCommand {
    #[clap(subcommand)]
    pub subcommand: FulfillmentSubcommand,
}

#[derive(Clap)]
pub enum FulfillmentSubcommand {
    /// Synchronize devices
    Sync(SyncCommand),
}

#[async_trait(?Send)]
impl ClientCommand for FulfillmentCommand {
    async fn run(&self, state: ClientCommandState) -> anyhow::Result<()> {
        match &self.subcommand {
            FulfillmentSubcommand::Sync(cmd) => cmd.run(state).await,
        }
    }
}
