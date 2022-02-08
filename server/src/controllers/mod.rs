pub mod meta;

use anyhow::Error;
use futures::Future;
use futures::FutureExt;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::accessory::Accessory;
use std::pin::Pin;
use tokio::sync::oneshot;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::IntoStaticStr)]
pub enum Name {
    Master,
    Meta,
}

#[derive(Debug)]
pub enum Message {
    Connected {
        accessory: Accessory,
    },
    Disconnected {
        accessory_id: accessory::ID,
    },

    Updated {
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    },
}

#[derive(Debug, Clone)]
pub struct Handle {
    sender: acu::Sender<Message>,
}

impl Handle {
    pub fn new(sender: acu::Sender<Message>) -> Self {
        Self { sender }
    }

    pub fn name(&self) -> &'static str {
        self.sender.name
    }

    pub async fn wait_for_stop(&self) {
        self.sender.closed().await;
    }

    pub async fn connected(&self, accessory: Accessory) {
        self.sender
            .notify(|| Message::Connected { accessory })
            .await
    }

    pub async fn disconnected(&self, accessory_id: accessory::ID) {
        self.sender
            .notify(|| Message::Disconnected { accessory_id })
            .await
    }

    pub async fn updated(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) {
        self.sender
            .notify(|| Message::Updated {
                accessory_id,
                service_name,
                characteristic,
            })
            .await
    }
}

pub struct Master {
    receiver: acu::Receiver<Message>,
    slave_controllers: Vec<Handle>,
}

impl<'s> Master {
    pub fn new(receiver: acu::Receiver<Message>) -> Self {
        Self {
            receiver,
            slave_controllers: vec![],
        }
    }

    pub fn insert(&mut self, handle: Handle) {
        self.slave_controllers.push(handle);
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await?;
        }
        Ok(())
    }

    async fn execute_for_all<'a>(
        &'s self,
        f: impl Fn(&'s Handle) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> + 'a,
    ) -> Result<(), Error> {
        use futures::stream::FuturesOrdered;
        use futures::StreamExt;

        let (controller_names, futures): (Vec<&'static str>, FuturesOrdered<_>) = self
            .slave_controllers
            .iter()
            .map(|controller| (controller.name(), f(controller)))
            .unzip();

        let results: Vec<Result<(), Error>> = futures.collect().await;
        for (result, controller) in results.iter().zip(controller_names.iter()) {
            match result {
                Ok(_) => tracing::debug!(?controller, "task completed"),
                Err(err) => tracing::error!(?controller, "task failed due to {}", err),
            };
        }
        Ok(())
    }

    async fn handle_message(&mut self, message: Message) -> Result<(), Error> {
        match message {
            Message::Connected { accessory } => {
                self.execute_for_all(|controller| {
                    let configured_accessory = accessory.to_owned();
                    async move {
                        controller.connected(configured_accessory).await;
                        Ok(())
                    }
                    .boxed()
                })
                .await?;
            }
            Message::Disconnected { accessory_id } => {
                self.execute_for_all(|controller| {
                    async move {
                        controller.disconnected(accessory_id).await;
                        Ok(())
                    }
                    .boxed()
                })
                .await?;
            }
            Message::Updated {
                accessory_id,
                service_name,
                characteristic,
            } => {
                self.execute_for_all(|controller| {
                    let service_name = service_name.to_owned();
                    let characteristic = characteristic.to_owned();
                    async move {
                        controller
                            .updated(accessory_id, service_name, characteristic)
                            .await;
                        Ok(())
                    }
                    .boxed()
                })
                .await?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SessionMessage {
    ReadCharacteristic {
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
        respond_to: oneshot::Sender<oneshot::Receiver<Result<Characteristic, accessory::Error>>>,
    },
    WriteCharacteristic {
        service_name: ServiceName,
        characteristic: Characteristic,
        respond_to: oneshot::Sender<oneshot::Receiver<Result<(), accessory::Error>>>,
    },
}

#[derive(Debug, Clone)]
pub struct SessionHandle {
    sender: acu::Sender<SessionMessage>,
}

impl SessionHandle {
    pub fn new(sender: acu::Sender<SessionMessage>) -> Self {
        Self { sender }
    }

    pub async fn wait_for_stop(&self) {
        self.sender.closed().await;
    }

    pub async fn read_characteristic(
        &self,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, accessory::Error> {
        self.sender
            .call(|oneshot| SessionMessage::ReadCharacteristic {
                service_name,
                characteristic_name,
                respond_to: oneshot,
            })
            .await
            .await
            .unwrap()
    }

    pub async fn write_characteristic(
        &self,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), accessory::Error> {
        self.sender
            .call(|oneshot| SessionMessage::WriteCharacteristic {
                service_name,
                characteristic,
                respond_to: oneshot,
            })
            .await
            .await
            .unwrap()
    }
}
