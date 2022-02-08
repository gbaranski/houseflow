use super::Handle;
use super::Message;
use super::Name;

pub struct FakeProvider {
    receiver: acu::Receiver<Message>,
}

impl FakeProvider {
    pub fn create() -> Handle {
        let (provider_sender, provider_receiver) = acu::channel(8, Name::Fake.into());
        let mut actor = Self {
            receiver: provider_receiver,
        };
        let handle = Handle::new(provider_sender);
        tokio::spawn(async move { actor.run().await });
        handle
    }

    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            tracing::info!("received {:?}", message);
        }
    }
}
