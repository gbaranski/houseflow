use crate::CommandContext;
use crate::Tokens;
use async_trait::async_trait;

pub struct Command {}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        let tokens = ctx.tokens.get()?;
        let refresh_token = ctx.refresh_token()?;
        let response = ctx.server_client()?.refresh_token(&refresh_token).await??;
        let tokens = Tokens {
            refresh: tokens.refresh,
            access: response.access_token,
        };
        ctx.tokens.save(&tokens)?;
        tracing::info!("✔ Succesfully refreshed token and saved to keystore");

        Ok(())
    }
}
