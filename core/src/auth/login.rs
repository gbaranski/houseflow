use crate::{ClientCommand, ClientConfig};
use async_trait::async_trait;
use auth_api::{Auth, KeystoreConfig};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct LoginCommand {
    /// Email used to log in, if not defined it will ask at runtime
    pub email: Option<String>,

    /// Password used to log in, if not defined it will ask at runtime
    pub password: Option<String>,
}

#[async_trait(?Send)]
impl ClientCommand for LoginCommand {
    async fn run(&self, cfg: ClientConfig) -> anyhow::Result<()> {
        use dialoguer::{Input, Password};
        use auth_types::LoginRequest;
        use types::UserAgent;

        let auth = Auth {
            url: cfg.auth_url.clone(),
            keystore: KeystoreConfig {
                path: cfg.keystore_path.clone().into(),
            },
        };

        let theme = crate::cli::get_theme();
        let email = match self.email {
            Some(ref email) => email.clone(),
            None => Input::with_theme(&theme)
                .with_prompt("Email")
                .interact_text()?,
        };

        let password = match self.password {
            Some(ref password) => password.clone(),
            None => Password::with_theme(&theme)
                .with_prompt("Password")
                .interact()?,
        };

        let login_request = LoginRequest {
            email,
            password,
            user_agent: UserAgent::Internal,
        };

        let login_response = auth.login(login_request.clone()).await??;
        log::info!("✔ Logged in as {}", login_request.email);
        auth.save_refresh_token(&login_response.refresh_token)
            .await?;
        log::debug!("Saved refresh token at {:?}", auth.keystore.path);

        Ok(())
    }
}
