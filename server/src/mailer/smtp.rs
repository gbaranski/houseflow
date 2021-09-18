use super::Error;
use async_trait::async_trait;
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
}

pub struct Mailer {
    transport: SmtpTransport,
    config: Config,
}

impl Mailer {
    pub fn new(config: Config) -> Self {
        let transport = lettre::SmtpTransport::relay(config.host.as_str())
            .unwrap()
            .port(config.port)
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
