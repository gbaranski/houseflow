use crate::{ClientCommandState, Command};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use houseflow_types::{
    auth::whoami,
    token::{AccessToken, RefreshToken},
};

use clap::Clap;

#[derive(Clap)]
pub struct StatusCommand {
    /// Display the auth token
    #[clap(long = "--show-token")]
    pub show_token: bool,
}

impl StatusCommand {
    async fn logged_in(
        &self,
        state: &ClientCommandState,
        whoami_response: whoami::ResponseBody,
    ) -> anyhow::Result<()> {
        let tokens = state.tokens.get().await?;
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
        let censor = |s: &str| std::iter::repeat("*").take(s.len()).collect();
        let (access_token, refresh_token) = match self.show_token {
            true => (access_token.to_string(), refresh_token.to_string()),
            false => (
                censor(&access_token.to_string()),
                censor(&refresh_token.to_string()),
            ),
        };

        println!("✔ Logged in");
        println!("  Username: {}", whoami_response.username);
        println!("  Email: {}", whoami_response.email);
        println!("  Keystore: {:#?}", state.tokens.path);
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

#[async_trait(?Send)]
impl Command<ClientCommandState> for StatusCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        let access_token = state.access_token().await?;

        let response = state.houseflow_api.whoami(&access_token).await??;
        self.logged_in(&state, response).await?;
        Ok(())
    }
}
