pub use super::Handle;

use super::Message;
use super::Name;
use houseflow_types::code::VerificationCode;
use tokio::sync::mpsc;

pub fn new(
    verification_code_sender: mpsc::UnboundedSender<(lettre::Address, VerificationCode)>,
) -> Handle {
    let (sender, receiver) = acu::channel(Name::Dummy);
    let mut actor = DummyMailer {
        receiver,
        verification_code_sender,
    };
    let handle = Handle { sender };
    tokio::spawn(async move { actor.run().await });
    handle
}

pub struct DummyMailer {
    receiver: acu::Receiver<Message, Name>,
    verification_code_sender: mpsc::UnboundedSender<(lettre::Address, VerificationCode)>,
}

impl DummyMailer {
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
            } => {
                self.verification_code_sender.send((to, code)).unwrap();
            }
        }
    }
}
