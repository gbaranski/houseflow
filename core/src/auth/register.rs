use crate::{ClientCommand, ClientConfig};
use async_trait::async_trait;
use auth_api::{Auth, KeystoreConfig};

use clap::Clap;

#[derive(Clap)]
pub struct RegisterCommand {
    /// Email used to register, if not defined it will ask at runtime
    pub email: Option<String>,

    /// Username used to register, if not defined it will ask at runtime
    pub username: Option<String>,

    /// Password used to register, if not defined it will ask at runtime
    pub password: Option<String>,
}

#[async_trait(?Send)]
impl ClientCommand for RegisterCommand {
    async fn run(&self, cfg: ClientConfig) -> anyhow::Result<()> {
    use dialoguer::{Input, Password};
    use auth_types::RegisterRequest;

    let auth = Auth {
        url: cfg.auth_url.clone(),
        keystore: KeystoreConfig {
            path: cfg.keystore_path.clone().into(),
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
