pub use super::Handle;

use super::Message;
use super::Name;

pub fn new() -> Handle {
    let (sender, receiver) = acu::channel(8, Name::Dummy);
    let mut actor = DummyProvider { receiver };
    let handle = Handle { sender };
    tokio::spawn(async move { actor.run().await });
    handle
}

pub struct DummyProvider {
    receiver: acu::Receiver<Message, Name>,
}

impl DummyProvider {
    async fn run(&mut self) {
        while let Some(message) = self.receiver.recv().await {
            tracing::info!("received {:?}", message);
        }
    }
}
