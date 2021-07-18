use crate::CommandContext;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use houseflow_types::token::{AccessToken, RefreshToken};

pub struct Command {
    pub show_token: bool,
}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, ctx: CommandContext) -> anyhow::Result<()> {
        let access_token = ctx.access_token().await?;

        let response = ctx.houseflow_api.whoami(&access_token).await??;
        let tokens = ctx.tokens.get().await?;
        let (access_token, refresh_token) = (
            AccessToken::decode_unsafe_novalidate(&tokens.access)?,
            RefreshToken::decode_unsafe_novalidate(&tokens.refresh)?,
        );

        let get_token_expiration = |exp_at: Option<&DateTime<Utc>>| match exp_at {
            Some(exp_at) => {
                use std::cmp::Ordering;

                match exp_at.cmp(&Utc::now()) {
                    Ordering::Equal => "just expired".to_string(),
                    Ordering::Greater => {
                        format!("expire at {}", exp_at.to_rfc2822())
                    }
                    Ordering::Less => {
                        format!("expired since {}", exp_at.to_rfc2822())
                    }
                }
            }
            None => "never".to_string(),
        };

        let (access_token_expiration, refresh_token_expiration) = (
            get_token_expiration(Some(&access_token.exp)),
            get_token_expiration(refresh_token.exp.as_ref()),
        );
        let censored = || std::iter::repeat("*").take(32).collect();
        let (access_token, refresh_token) = match self.show_token {
            true => (access_token.to_string(), refresh_token.to_string()),
            false => (censored(), censored()),
        };

        println!("✔ Logged in");
        println!("  Username: {}", response.username);
        println!("  Email: {}", response.email);
        println!("  Keystore: {:#?}", ctx.tokens.path);
        println!(
            "  Access token({}): {}",
            access_token_expiration, access_token
        );
        println!(
            "  Refresh token({}): {}",
            refresh_token_expiration, refresh_token
        );

        Ok(())
    }
}
