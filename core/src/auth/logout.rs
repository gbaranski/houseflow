use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct LogoutCommand {}

#[async_trait(?Send)]
impl Command<ClientCommandState> for LogoutCommand {
    async fn run(&self, state: ClientCommandState) -> anyhow::Result<()> {
        let tokens = state.tokens.get().await?;

        state.houseflow_api.logout(&tokens.refresh).await?.into_result()?;

        state.tokens.flush().await?;
        log::info!("✔ Succesfully logged out");

        Ok(())
    }
}
