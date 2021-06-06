use crate::Opt;
use houseflow_auth_api::{Auth, KeystoreConfig};
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Command {
    /// Log in to existing Houseflow account
    Login(LoginCommand),

    /// Register a new Houseflow account
    Register(RegisterCommand),
}

pub async fn auth(opt: &Opt, command: &Command) -> anyhow::Result<()> {
    match command {
        Command::Login(command) => login(opt, command).await,
        Command::Register(command) => register(opt, command).await,
    }
}

#[derive(StructOpt)]
pub struct LoginCommand {
    /// Email used to log in, if not defined it will ask at runtime
    pub email: Option<String>,

    /// Password used to log in, if not defined it will ask at runtime
    pub password: Option<String>,
}

pub async fn login(opt: &Opt, command: &LoginCommand) -> anyhow::Result<()> {
    use dialoguer::{Input, Password};
    use houseflow_auth_types::LoginRequest;
    use houseflow_types::UserAgent;

    let auth = Auth {
        url: opt.auth_url.clone(),
        keystore: KeystoreConfig {
            path: opt.keystore_path.clone().into(),
        },
    };

    let theme = crate::cli::get_theme();
    let email = match command.email {
        Some(ref email) => email.clone(),
        None => Input::with_theme(&theme)
            .with_prompt("Email")
            .interact_text()?,
    };

    let password = match command.password {
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

    let login_response = auth.login(login_request.clone()).await?;
    log::info!("✔ Logged in as {}", login_request.email);
    auth.save_refresh_token(&login_response.refresh_token)
        .await?;
    log::debug!("Saved refresh token at {:?}", auth.keystore.path);

    Ok(())
}

#[derive(StructOpt)]
pub struct RegisterCommand {
    /// Email used to register, if not defined it will ask at runtime
    pub email: Option<String>,

    /// Username used to register, if not defined it will ask at runtime
    pub username: Option<String>,

    /// Password used to register, if not defined it will ask at runtime
    pub password: Option<String>,
}

pub async fn register(opt: &Opt, command: &RegisterCommand) -> anyhow::Result<()> {
    use dialoguer::{Input, Password};
    use houseflow_auth_types::RegisterRequest;

    let auth = Auth {
        url: opt.auth_url.clone(),
        keystore: KeystoreConfig {
            path: opt.keystore_path.clone().into(),
        },
    };
    let theme = crate::cli::get_theme();

    let username = match command.username {
        Some(ref username) => username.clone(),
        None => Input::with_theme(&theme)
            .with_prompt("Username")
            .interact()?,
    };
    let email = match command.email {
        Some(ref email) => email.clone(),
        None => Input::with_theme(&theme)
            .with_prompt("Email")
            .interact_text()?,
    };


    let password = match command.password {
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
