use crate::{ClientCommandState, Command, Tokens};
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
impl Command<ClientCommandState> for LoginCommand {
    async fn run(&self, state: ClientCommandState) -> anyhow::Result<()> {
        use houseflow_types::auth::LoginRequest;

        use dialoguer::{Input, Password};
        use houseflow_types::UserAgent;

        let theme = crate::cli::get_dialoguer_theme();
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

        let login_response = state
            .houseflow_api
            .login(login_request.clone())
            .await?
            .into_result()?;
        log::info!("✔ Logged in as {}", login_request.email);
        let tokens = Tokens {
            refresh: login_response.refresh_token,
            access: login_response.access_token,
        };
        state.tokens.save(&tokens).await?;
        log::debug!("Saved refresh token at {:#?}", state.tokens.path);

        Ok(())
    }
}
