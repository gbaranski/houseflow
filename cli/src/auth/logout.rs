use crate::CommandContext;
use async_trait::async_trait;

pub struct Command {}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, ctx: CommandContext) -> anyhow::Result<()> {
        ctx.tokens.remove().await?;
        tracing::info!("✔ Succesfully logged out");

        Ok(())
    }
}
