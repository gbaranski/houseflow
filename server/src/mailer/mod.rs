pub mod noop;
pub mod ses;

use lettre::Message;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("aws-ses send raw email error: {0}")]
    AwsSesSendRawEmailError(#[from] rusoto_core::RusotoError<rusoto_ses::SendRawEmailError>)
}

#[async_trait]
pub trait Mailer: Send + Sync {
    async fn send(&self, message: Message) -> Result<(), Error>;

    async fn send_verification_link(&self, address: String) {
        todo!();
    }
}
