use crate::{ClientCommand, ClientCommandState};
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct LogoutCommand {}

#[async_trait(?Send)]
impl ClientCommand for LogoutCommand {
    async fn run(&self, state: ClientCommandState) -> anyhow::Result<()> {
        let keystore_file = state.keystore.read().await?;

        state
            .auth
            .logout(&keystore_file.refresh_token)
            .await?
            .into_result()?;

        state.keystore.remove().await?;
        log::info!("✔ Succesfully logged out");

        Ok(())
    }
}
