use crate::CommandContext;
use async_trait::async_trait;

pub struct Command {}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, ctx: CommandContext) -> anyhow::Result<()> {
        let refresh_token = ctx.refresh_token().await?;

        ctx.houseflow_api.logout(&refresh_token).await??;

        ctx.tokens.remove().await?;
        tracing::info!("✔ Succesfully logged out");

        Ok(())
    }
}
