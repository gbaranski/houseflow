use crate::{CommandContext, Tokens};
use async_trait::async_trait;

pub struct Command {
    pub email: String,
    pub password: String,
}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, ctx: CommandContext) -> anyhow::Result<()> {
        use houseflow_types::auth::login;

        let login_request = login::Request {
            email: self.email,
            password: self.password,
        };

        let login_response = ctx.houseflow_api.login(&login_request).await??;

        tracing::info!("✔ Logged in as {}", login_request.email);
        let tokens = Tokens {
            refresh: login_response.refresh_token,
            access: login_response.access_token,
        };
        ctx.tokens.save(&tokens).await?;
        tracing::debug!("Saved refresh token at {:#?}", ctx.tokens.path);

        Ok(())
    }
}
