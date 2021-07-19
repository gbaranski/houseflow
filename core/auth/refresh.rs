use crate::{CommandContext, Tokens};
use async_trait::async_trait;

pub struct Command {}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        let tokens = ctx.tokens.get().await?;
        let refresh_token = ctx.refresh_token().await?;
        let response = ctx
            .houseflow_api().await?
            .refresh_token(&refresh_token)
            .await??;
        let tokens = Tokens {
            refresh: tokens.refresh,
            access: response.access_token,
        };
        ctx.tokens.save(&tokens).await?;
        tracing::info!("✔ Succesfully refreshed token and saved to keystore");

        Ok(())
    }
}
