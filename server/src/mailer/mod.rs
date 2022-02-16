pub mod dummy;
pub mod smtp;

use houseflow_types::code::VerificationCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("smtp: {0}")]
    Smtp(#[from] lettre::transport::smtp::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::IntoStaticStr)]
pub enum Name {
    Master,
    Dummy,
    Smtp,
}

impl acu::MasterName for Name {
    fn master_name() -> Self {
        Self::Master
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SendVerificationCode {
        subject: String,
        to: lettre::Address,
        code: VerificationCode,
    },
}

impl acu::Message for Message {}

use async_trait::async_trait;

#[async_trait]
pub trait MailerExt {
    async fn send_verification_code(
        &self,
        subject: String,
        to: lettre::Address,
        code: VerificationCode,
    );
}

#[async_trait]
impl MailerExt for Handle {
    async fn send_verification_code(
        &self,
        subject: String,
        to: lettre::Address,
        code: VerificationCode,
    ) {
        self.sender
            .notify(Message::SendVerificationCode { subject, to, code })
            .await
    }
}

pub type Handle = acu::Handle<Message, Name>;

pub type MasterHandle = acu::BroadcasterMasterHandle<Message, Name>;

impl From<Error> for houseflow_types::errors::ServerError {
    fn from(val: Error) -> Self {
        houseflow_types::errors::InternalError::Mailer(val.to_string()).into()
    }
}
