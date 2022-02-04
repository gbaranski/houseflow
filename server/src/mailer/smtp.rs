use super::Handle;
use super::Message;
use super::Name;
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use lettre::Transport;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: lettre::message::Mailbox,
}

pub struct Mailer {
    receiver: acu::Receiver<Message>,
    config: Config,
    transport: SmtpTransport,
}

impl Mailer {
    pub fn create(config: Config) -> Handle {
        let (sender, receiver) = acu::channel(8, Name::Smtp.into());
        let transport = lettre::SmtpTransport::relay(config.host.as_str())
            .unwrap()
            .port(config.port)
            .credentials(Credentials::new(
                config.username.clone(),
                config.password.clone(),
            ))
            .build();
        let mut actor = Self {
            receiver,
            config,
            transport,
        };
        let handle = Handle::new(Name::Smtp, sender);
        tokio::spawn(async move { actor.run().await });
        handle
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            self.handle_message(message).await;
        }
    }

    async fn handle_message(&mut self, message: Message) {
        match message {
            Message::SendVerificationCode {
                subject,
                to,
                code,
                respond_to,
            } => {
                let body = format!(
                    "Your verification code: {}. It will be valid for next 30 minutes. Hurry up!",
                    code
                );
                let message = lettre::Message::builder()
                    .from(self.config.from.to_owned())
                    .to(to)
                    .subject(subject)
                    .body(body)
                    .unwrap();

                let response = self
                    .transport
                    .send(&message)
                    .map(|_| ())
                    .map_err(Into::into);
                respond_to.send(response).unwrap();
            }
        }
    }
}
