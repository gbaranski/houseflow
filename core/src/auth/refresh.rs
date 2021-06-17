use crate::{Command, ClientCommandState, KeystoreFile};
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct RefreshCommand {}

#[async_trait(?Send)]
impl Command<ClientCommandState> for RefreshCommand {
    async fn run(&self, state: ClientCommandState) -> anyhow::Result<()> {
        let keystore_file = state.keystore.read().await?;
        let response = state
            .auth
            .fetch_access_token(&keystore_file.refresh_token)
            .await?
            .into_result()?;
        let new_keystore_file = KeystoreFile {
            refresh_token: keystore_file.refresh_token,
            access_token: response.access_token,
        };
        state.keystore.save(&new_keystore_file).await?;
        log::info!("✔ Succesfully refreshed token and saved to keystore");

        Ok(())
    }
}
