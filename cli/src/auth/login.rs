use crate::CommandContext;
use crate::Tokens;
use async_trait::async_trait;
use houseflow_types::code::VerificationCode;

pub struct Command {
    pub email: String,
    pub code: Option<VerificationCode>,
}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        use houseflow_types::auth::login;

        match self.code {
            Some(code) => {
                let request = login::Request {
                    email: self.email,
                    verification_code: Some(code),
                };
                let response = ctx.server_client()?.login(&request).await??;
                match response {
                    login::Response::LoggedIn {
                        access_token,
                        refresh_token,
                    } => {
                        tracing::info!("✔ Logged in as {}", request.email);
                        let tokens = Tokens {
                            refresh: refresh_token,
                            access: access_token,
                        };
                        ctx.tokens.save(&tokens)?;
                        tracing::debug!("Saved refresh token at {:#?}", ctx.tokens.path);
                    }
                    _ => panic!("Expected Response::LoggedIn"),
                };
            }
            None => {
                let request = login::Request {
                    email: self.email,
                    verification_code: None,
                };
                let response = ctx.server_client()?.login(&request).await??;
                match response {
                    login::Response::VerificationCodeSent => {
                        tracing::info!(
                            "✔ Verification code sent to {}. Please copy the code and re-run the command with --code <code-from-email>",
                            request.email
                        );
                    }
                    _ => panic!("Expected Response::VerificationCodeSent"),
                };
            }
        };

        Ok(())
    }
}
