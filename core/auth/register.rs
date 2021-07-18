use crate::CommandContext;
use async_trait::async_trait;

pub struct Command {
    /// Username used to register, if not defined it will ask at runtime
    pub username: Option<String>,

    /// Email used to register, if not defined it will ask at runtime
    pub email: Option<String>,

    /// Password used to register, if not defined it will ask at runtime
    pub password: Option<String>,
}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, ctx: CommandContext) -> anyhow::Result<()> {
        use dialoguer::{Input, Password};
        use houseflow_types::auth::register;

        let theme = crate::dialoguer_theme();

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

        let register_request = register::Request {
            email,
            username,
            password,
        };

        ctx.houseflow_api.register(&register_request).await??;
        tracing::info!("✔ Created new account");

        Ok(())
    }
}
