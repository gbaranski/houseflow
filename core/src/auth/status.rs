use crate::{ClientCommand, ClientConfig};
use async_trait::async_trait;
use auth_api::{Auth, KeystoreConfig};
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
    fn no_saved_credentials(&self, cfg: ClientConfig) {
        println!(
            "❌ No saved credentials at {}",
            cfg.keystore_path.to_str().unwrap_or("none")
        );
    }

    fn logged_in(&self, cfg: ClientConfig, whoami_response: WhoamiResponseBody, token: Token) {
        let token = match self.show_token {
            true => token.to_string(),
            false => std::iter::repeat("*")
                .take(token.to_string().len())
                .collect(),
        };

        println!("✔ Logged in");
        println!("  Username: {}", whoami_response.username);
        println!("  Email: {}", whoami_response.email);
        println!(
            "  Token({}): {}",
            cfg.keystore_path.to_str().unwrap_or("none"),
            token
        );
    }

    fn error(&self, error: WhoamiResponseError) {
        println!("❌ Error: {}", error);
    }
}

#[async_trait(?Send)]
impl ClientCommand for StatusCommand {
    async fn run(&self, cfg: ClientConfig) -> anyhow::Result<()> {
        let auth = Auth {
            url: cfg.auth_url.clone(),
            keystore: KeystoreConfig {
                path: cfg.keystore_path.clone(),
            },
        };
        let refresh_token = match auth.read_refresh_token().await? {
            Some(token) => token,
            None => {
                return {
                    self.no_saved_credentials(cfg);
                    Ok(())
                }
            }
        };
        let access_token = auth
            .fetch_access_token(&refresh_token)
            .await?
            .into_result()?
            .access_token;

        match auth.whoami(&access_token).await? {
            WhoamiResponse::Ok(response) => self.logged_in(cfg, response, access_token),
            WhoamiResponse::Err(err) => self.error(err),
        };
        Ok(())
    }
}
