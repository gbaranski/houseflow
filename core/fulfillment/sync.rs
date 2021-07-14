use crate::{ClientCommandState, Command};
use anyhow::Context;
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct SyncCommand {}

#[async_trait(?Send)]
impl Command<ClientCommandState> for SyncCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        let access_token = state.access_token().await?;
        let response = state
            .houseflow_api
            .sync(&access_token)
            .await??;

        println!("Synced {} devices", response.devices.len());
        response.devices.iter().for_each(|device| {
            println!(
                "Device ID: {}, Name: {}",
                device.id.to_string(),
                device.name
            )
        });
        state
            .devices
            .save(&response.devices)
            .await
            .with_context(|| "save devices")?;
        tracing::debug!("saved devices to {:#?}", state.devices.path);

        Ok(())
    }
}
