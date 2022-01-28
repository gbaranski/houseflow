pub mod fake;
pub mod smtp;

use houseflow_types::code::VerificationCode;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum Name {
    Master,
    Fake,
    Smtp,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("smtp: {0}")]
    Smtp(#[from] lettre::transport::smtp::Error),
}

#[derive(Debug)]
pub enum Message {
    SendVerificationCode {
        subject: String,
        to: lettre::message::Mailbox,
        code: VerificationCode,
        respond_to: oneshot::Sender<Result<(), Error>>,
    },
}

#[derive(Debug, Clone)]
pub struct Handle {
    name: Name,
    sender: mpsc::Sender<Message>,
}

impl Handle {
    async fn call<R>(&self, message_fn: impl FnOnce(oneshot::Sender<R>) -> Message) -> R {
        let (tx, rx) = oneshot::channel();
        let message = message_fn(tx);
        tracing::debug!("calling {:?} on a mailer named {}", message, self.name);
        self.sender.send(message).await.unwrap();
        rx.await.unwrap()
    }
}

impl Handle {
    pub fn new(name: Name, sender: mpsc::Sender<Message>) -> Self {
        Self { name, sender }
    }

    pub async fn send_verification_code(
        &self,
        subject: String,
        to: lettre::message::Mailbox,
        code: VerificationCode,
    ) -> Result<(), Error> {
        self.call(|respond_to| Message::SendVerificationCode {
            subject,
            to,
            code,
            respond_to,
        })
        .await
    }
}

impl From<Error> for houseflow_types::errors::ServerError {
    fn from(val: Error) -> Self {
        houseflow_types::errors::InternalError::Mailer(val.to_string()).into()
    }
}
