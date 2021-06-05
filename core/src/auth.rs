use crate::Opt;
use houseflow_auth_api::{Auth, AuthConfig, TokenStoreConfig};

pub async fn login(opt: Opt) -> anyhow::Result<()> {
    use dialoguer::{Input, Password};
    use houseflow_auth_types::LoginRequest;
    use houseflow_types::UserAgent;

    let auth_config = AuthConfig {
        url: opt.auth_url,
        token_store: TokenStoreConfig {
            path: opt.token_store_path,
        },
    };
    let auth = Auth::new(auth_config.clone());
    let theme = crate::cli::get_theme();
    let email: String = Input::with_theme(&theme)
        .with_prompt("Email")
        .interact_text()?;

    let password: String = Password::with_theme(&theme)
        .with_prompt("Password")
        .interact()?;

    let login_request = LoginRequest {
        email,
        password,
        user_agent: UserAgent::Internal,
    };

    let login_response = auth.login(login_request).await?;
    log::info!("Succesfully logged in");
    auth.save_refresh_token(&login_response.refresh_token)
        .await?;
    log::debug!("Saved refresh token at {:?}", auth_config.token_store.path);

    Ok(())
}
