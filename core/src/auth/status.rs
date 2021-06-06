use crate::{Command, Opt};
use async_trait::async_trait;
use houseflow_auth_api::{Auth, KeystoreConfig};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct StatusCommand {
    /// Email used to register, if not defined it will ask at runtime
    pub email: Option<String>,

    /// Username used to register, if not defined it will ask at runtime
    pub username: Option<String>,

    /// Password used to register, if not defined it will ask at runtime
    pub password: Option<String>,
}

#[async_trait(?Send)]
impl Command for StatusCommand {
    async fn run(&self, opt: &Opt) -> anyhow::Result<()> {
        use dialoguer::{Input, Password};
        use houseflow_auth_types::RegisterRequest;

        let auth = Auth {
            url: opt.auth_url.clone(),
            keystore: KeystoreConfig {
                path: opt.keystore_path.clone().into(),
            },
        };
        let theme = crate::cli::get_theme();

        let username = match self.username {
            Some(ref username) => username.clone(),
            None => Input::with_theme(&theme)
                .with_prompt("Username")
                .interact()?,
        };
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

        let register_request = RegisterRequest {
            email,
            password,
            username,
        };

        auth.register(register_request).await??;
        log::info!("✔ Created new account");

        Ok(())
    }
}
