use crate::{ClientCommand, ClientCommandState, KeystoreFile};
use async_trait::async_trait;
use auth_types::{WhoamiResponse, WhoamiResponseBody, WhoamiResponseError};

use clap::Clap;

#[derive(Clap)]
pub struct StatusCommand {
    /// Display the auth token
    #[clap(long = "--show-token")]
    pub show_token: bool,
}

impl StatusCommand {
    fn logged_in(
        &self,
        state: &ClientCommandState,
        whoami_response: WhoamiResponseBody,
        keystore_file: KeystoreFile,
    ) {
        let (access_token, refresh_token) =
            (keystore_file.access_token, keystore_file.refresh_token);
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
        println!("  Access token: {}", access_token);
        println!("  Refresh token: {}", refresh_token);
    }

    fn error(&self, error: WhoamiResponseError) {
        println!("❌ Error: {}", error);
    }
}

#[async_trait(?Send)]
impl ClientCommand for StatusCommand {
    async fn run(&self, state: ClientCommandState) -> anyhow::Result<()> {
        let keystore_file = state.keystore.read().await?;

        match state.auth.whoami(&keystore_file.access_token).await? {
            WhoamiResponse::Ok(response) => self.logged_in(&state, response, keystore_file),
            WhoamiResponse::Err(err) => self.error(err),
        };
        Ok(())
    }
}
