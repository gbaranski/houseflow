use crate::{ClientCommand, KeystoreFile, ClientCommandState};
use async_trait::async_trait;


use clap::Clap;

#[derive(Clap)]
pub struct LoginCommand {
    /// Email used to log in, if not defined it will ask at runtime
    pub email: Option<String>,

    /// Password used to log in, if not defined it will ask at runtime
    pub password: Option<String>,
}

#[async_trait(?Send)]
impl ClientCommand for LoginCommand {
    async fn run(&self, state: ClientCommandState) -> anyhow::Result<()> {
        use auth::types::LoginRequest;
        use dialoguer::{Input, Password};
        use types::UserAgent;

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

        let login_response = state.auth.login(login_request.clone()).await?.into_result()?;
        log::info!("✔ Logged in as {}", login_request.email);
        let keystore_file = KeystoreFile {
            refresh_token: login_response.refresh_token,
            access_token: login_response.access_token,
        };
        state.keystore.save(&keystore_file).await?;
        log::debug!("Saved refresh token at {:?}", state.keystore.path);

        Ok(())
    }
}
