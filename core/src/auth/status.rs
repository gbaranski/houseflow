use crate::{Command, Opt};
use async_trait::async_trait;
use auth_api::{Auth, KeystoreConfig};
use auth_types::{WhoamiError, WhoamiResponseBody};
use token::Token;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct StatusCommand {
    /// Display the auth token
    #[structopt(short = "t", long = "--show-token")]
    pub show_token: bool,
}

impl StatusCommand {
    fn no_saved_credentials(&self, opt: &Opt) {
        println!("❌ No saved credentials at {}", opt.keystore_path);
    }

    fn logged_in(&self, opt: &Opt, whoami_response: WhoamiResponseBody, token: Token) {
        let token = match self.show_token {
            true => token.to_string(),
            false => std::iter::repeat("*")
                .take(token.to_string().len())
                .collect(),
        };

        println!("✔ Logged in");
        println!("  Username: {}", whoami_response.username);
        println!("  Email: {}", whoami_response.email);
        println!("  Token({}): {}", opt.keystore_path, token);
    }

    fn error(&self, error: WhoamiError) {
        println!("❌ Error: {}", error);
    }
}

#[async_trait(?Send)]
impl Command for StatusCommand {
    async fn run(&self, opt: &Opt) -> anyhow::Result<()> {
        let auth = Auth {
            url: opt.auth_url.clone(),
            keystore: KeystoreConfig {
                path: opt.keystore_path.clone().into(),
            },
        };
        let token = match auth.read_refresh_token().await? {
            Some(token) => token,
            None => {
                return {
                    self.no_saved_credentials(opt);
                    Ok(())
                }
            }
        };

        match auth.whoami(&token).await? {
            Ok(response) => self.logged_in(opt, response, token),
            Err(err) => self.error(err),
        };
        Ok(())
    }
}
