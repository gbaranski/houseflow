use lettre::Message;
use async_trait::async_trait;
use tokio::sync::mpsc;
use super::Error;

pub struct Mailer {
    tx: mpsc::UnboundedSender<Message>
}

impl Mailer {
    pub fn new(tx: mpsc::UnboundedSender<Message>) -> Self {
        Self {tx}
    }
}

#[async_trait]
impl super::Mailer for Mailer {
    async fn send(&self, email: Message) -> Result<(), Error> {
        self.tx.send(email).unwrap();
        Ok(())
    }
}
