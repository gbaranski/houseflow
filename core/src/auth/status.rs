use crate::{ClientCommand, ClientCommandState};
use async_trait::async_trait;
use auth_types::{WhoamiResponse, WhoamiResponseBody, WhoamiResponseError};
use token::Token;

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
        whoami_response: WhoamiResponseBody,
    ) -> anyhow::Result<()> {
        let keystore_file = state.keystore.read().await?;
        let (access_token, refresh_token) =
            (keystore_file.access_token, keystore_file.refresh_token);

        let get_token_expiration = |token: &Token| match token.expires_at().as_ref() {
            Some(expiration_date) => humantime::Duration::from(std::time::Duration::from_secs(
                expiration_date
                    .duration_since(std::time::SystemTime::now())
                    .unwrap()
                    .as_secs(),
            ))
            .to_string(),
            None => "never".to_string(),
        };

        let (access_token_expiration, refresh_token_expiration) = (
            get_token_expiration(&access_token),
            get_token_expiration(&refresh_token),
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
        println!(
            "  Keystore: {}",
            state
                .config
                .keystore_path
                .to_str()
                .unwrap_or("INVALID_PATH")
        );
        println!(
            "  Access token(valid for: {}): {}",
            access_token_expiration, access_token
        );
        println!(
            "  Refresh token(valid for: {}): {}",
            refresh_token_expiration, refresh_token
        );

        Ok(())
    }

    fn error(&self, error: WhoamiResponseError) {
        println!("❌ Error: {}", error);
    }
}

#[async_trait(?Send)]
impl ClientCommand for StatusCommand {
    async fn run(&self, state: ClientCommandState) -> anyhow::Result<()> {
        let access_token = state.access_token().await?;

        match state.auth.whoami(&access_token).await? {
            WhoamiResponse::Ok(response) => self.logged_in(&state, response).await?,
            WhoamiResponse::Err(err) => self.error(err),
        };
        Ok(())
    }
}
