use super::Handle;
use super::Message;
use super::Name;
use houseflow_types::code::VerificationCode;
use tokio::sync::mpsc;

pub struct Mailer {
    receiver: acu::Receiver<Message>,
    verification_code_sender: mpsc::Sender<(lettre::message::Mailbox, VerificationCode)>,
}

impl Mailer {
    pub fn create(
        verification_code_sender: mpsc::Sender<(lettre::message::Mailbox, VerificationCode)>,
    ) -> Handle {
        let (sender, receiver) = acu::channel(8, Name::Fake.into());
        let mut actor = Self {
            receiver,
            verification_code_sender,
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
                subject: _,
                to,
                code,
                respond_to,
            } => {
                self.verification_code_sender
                    .send((to, code))
                    .await
                    .unwrap();
                respond_to.send(Ok(())).unwrap();
            }
        }
    }
}
