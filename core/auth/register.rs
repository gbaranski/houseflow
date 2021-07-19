use crate::CommandContext;
use async_trait::async_trait;

pub type Command = houseflow_types::auth::register::Request;

#[async_trait]
impl crate::Command for Command {
    async fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        ctx.houseflow_api().await?.register(&self).await??;
        tracing::info!("✔ Created new account");

        Ok(())
    }
}
