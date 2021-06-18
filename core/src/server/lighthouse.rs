use crate::{Command, ServerCommandState};
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct RunLighthouseCommand {}

#[async_trait(?Send)]
impl Command<ServerCommandState> for RunLighthouseCommand {
    async fn run(&self, state: ServerCommandState) -> anyhow::Result<()> {
        lighthouse::server::run(state.config.lighthouse).await?;

        Ok(())
    }
}
