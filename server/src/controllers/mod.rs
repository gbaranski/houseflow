use anyhow::Error;
use futures::Future;
use futures::FutureExt;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::accessory::Accessory;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum Name {
    Master,
}

#[derive(Debug)]
pub enum Message {
    Connected {
        configured_accessory: Accessory,
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
    pub name: Name,
    sender: mpsc::Sender<Message>,
}

impl Handle {
    pub fn new(name: Name, sender: mpsc::Sender<Message>) -> Self {
        Self { name, sender }
    }

    pub async fn wait_for_stop(&self) {
        self.sender.closed().await;
    }

    pub async fn connected(&self, configured_accessory: Accessory) {
        self.notify(|| Message::Connected {
            configured_accessory,
        })
        .await
    }

    pub async fn disconnected(&self, accessory_id: accessory::ID) {
        self.notify(|| Message::Disconnected { accessory_id }).await
    }

    pub async fn updated(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) {
        self.notify(|| Message::Updated {
            accessory_id,
            service_name,
            characteristic,
        })
        .await
    }
}

impl Handle {
    async fn notify(&self, message_fn: impl FnOnce() -> Message) {
        let message = message_fn();
        tracing::debug!("notify {:?} on a controller named {}", message, self.name);
        self.sender.send(message).await.unwrap();
    }
}

pub struct Master {
    receiver: mpsc::Receiver<Message>,
    slave_controllers: Vec<Handle>,
}

impl<'s> Master {
    pub fn new(receiver: mpsc::Receiver<Message>) -> Self {
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

        let (controller_names, futures): (Vec<&Name>, FuturesOrdered<_>) = self
            .slave_controllers
            .iter()
            .map(|controller| (&controller.name, f(controller)))
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
            Message::Connected {
                configured_accessory,
            } => {
                self.execute_for_all(|controller| {
                    let configured_accessory = configured_accessory.to_owned();
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
    sender: mpsc::Sender<SessionMessage>,
}

impl SessionHandle {
    pub fn new(sender: mpsc::Sender<SessionMessage>) -> Self {
        Self { sender }
    }
    pub async fn wait_for_stop(&self) {
        self.sender.closed().await;
    }

    async fn call<R>(&self, message_fn: impl FnOnce(oneshot::Sender<R>) -> SessionMessage) -> R {
        let (tx, rx) = oneshot::channel();
        let message = message_fn(tx);
        tracing::debug!("calling {:?} on a session", message);
        self.sender.send(message).await.unwrap();
        rx.await.unwrap()
    }

    pub async fn read_characteristic(
        &self,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, accessory::Error> {
        self.call(|oneshot| SessionMessage::ReadCharacteristic {
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
        self.call(|oneshot| SessionMessage::WriteCharacteristic {
            service_name,
            characteristic,
            respond_to: oneshot,
        })
        .await
        .await
        .unwrap()
    }
}
