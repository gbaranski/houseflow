pub mod fake;
pub mod smtp;

use houseflow_types::code::VerificationCode;
use tokio::sync::oneshot;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::IntoStaticStr)]
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
    sender: acu::Sender<Message>,
}

impl Handle {
    pub fn new(name: Name, sender: acu::Sender<Message>) -> Self {
        Self { name, sender }
    }

    pub async fn send_verification_code(
        &self,
        subject: String,
        to: lettre::message::Mailbox,
        code: VerificationCode,
    ) -> Result<(), Error> {
        self.sender.call(|respond_to| Message::SendVerificationCode {
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
