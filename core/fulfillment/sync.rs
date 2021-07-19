use crate::CommandContext;
use anyhow::Context;
use async_trait::async_trait;

pub struct Command {}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        let access_token = ctx.access_token().await?;
        let response = ctx.houseflow_api().await?.sync(&access_token).await??;

        println!("Synced {} devices", response.devices.len());
        response.devices.iter().for_each(|device| {
            println!(
                "Device ID: {}, Name: {}",
                device.id.to_string(),
                device.name
            )
        });
        ctx.devices
            .save(&response.devices)
            .await
            .with_context(|| "save devices")?;
        tracing::debug!("saved devices to {:#?}", ctx.devices.path);

        Ok(())
    }
}
