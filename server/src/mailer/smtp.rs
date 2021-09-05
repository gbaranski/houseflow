use super::Error;
use async_trait::async_trait;
use houseflow_config::server::Smtp as Config;
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;

pub struct Mailer {
    transport: SmtpTransport,
    config: Config,
}

impl Mailer {
    pub async fn new(config: Config) -> Self {
        let transport = lettre::SmtpTransport::relay(config.hostname.to_string().as_str())
            .unwrap()
            .credentials(Credentials::new(
                config.username.clone(),
                config.password.clone(),
            ))
            .build();
        Self { transport, config }
    }
}

#[async_trait]
impl super::Mailer for Mailer {
    async fn send(&self, email: Message) -> Result<(), Error> {
        self.transport.send(&email)?;
        Ok(())
    }

    fn from_address(&self) -> &str {
        &self.config.from
    }
}
