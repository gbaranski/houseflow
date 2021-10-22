use crate::CommandContext;
use crate::Tokens;

pub struct Command {}

impl crate::Command for Command {
    fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        let tokens = ctx.tokens.get()?;
        let refresh_token = ctx.refresh_token()?;
        let response = ctx.client()?.refresh_token(&refresh_token)??;
        let tokens = Tokens {
            refresh: tokens.refresh,
            access: response.access_token,
        };
        ctx.tokens.save(&tokens)?;
        tracing::info!("✔ Succesfully refreshed token and saved to keystore");

        Ok(())
    }
}
