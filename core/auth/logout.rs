use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct LogoutCommand {}

#[async_trait(?Send)]
impl Command<ClientCommandState> for LogoutCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        let refresh_token = state.refresh_token().await?;

        state
            .houseflow_api
            .logout(&refresh_token)
            .await??;

        state.tokens.flush().await?;
        log::info!("✔ Succesfully logged out");

        Ok(())
    }
}
