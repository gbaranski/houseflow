use crate::CommandContext;
use anyhow::Context;

pub struct Command {}

impl crate::Command for Command {
    fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        let access_token = ctx.access_token()?;
        let response = ctx.client()?.sync(&access_token)??;

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
            .with_context(|| "save devices")?;
        tracing::debug!("saved devices to {:#?}", ctx.devices.path);

        Ok(())
    }
}
