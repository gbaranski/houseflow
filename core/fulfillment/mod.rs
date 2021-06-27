mod execute;
mod sync;
mod query;

use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use execute::ExecuteCommand;
use sync::SyncCommand;
use query::QueryCommand;

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

    /// Execute command on device
    Execute(ExecuteCommand),

    /// Query state of the device
    Query(QueryCommand),
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for FulfillmentCommand {
    async fn run(&self, state: ClientCommandState) -> anyhow::Result<()> {
        match &self.subcommand {
            FulfillmentSubcommand::Sync(cmd) => cmd.run(state).await,
            FulfillmentSubcommand::Execute(cmd) => cmd.run(state).await,
            FulfillmentSubcommand::Query(cmd) => cmd.run(state).await,
        }
    }
}
