pub mod lighthouse;

use anyhow::Error;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum Name {
    Master,
    Lighthouse,
}

#[derive(Debug)]
pub enum Message {
    ReadCharacteristic {
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
        respond_to: oneshot::Sender<Result<Characteristic, accessory::Error>>,
    },
    WriteCharacteristic {
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
        respond_to: oneshot::Sender<Result<(), accessory::Error>>,
    },
    GetAccessories {
        respond_to: oneshot::Sender<Vec<accessory::ID>>,
    },
    IsConnected {
        accessory_id: accessory::ID,
        respond_to: oneshot::Sender<bool>,
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

    pub async fn write_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), accessory::Error> {
        self.call(|respond_to| Message::WriteCharacteristic {
            accessory_id,
            service_name,
            characteristic,
            respond_to,
        })
        .await
    }

    pub async fn read_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, accessory::Error> {
        self.call(|respond_to| Message::ReadCharacteristic {
            accessory_id,
            service_name,
            characteristic_name,
            respond_to,
        })
        .await
    }

    pub async fn get_accessories(&self) -> Vec<accessory::ID> {
        self.call(|respond_to| Message::GetAccessories { respond_to })
            .await
    }

    pub async fn is_connected(&self, accessory_id: accessory::ID) -> bool {
        self.call(|respond_to| Message::IsConnected {
            accessory_id,
            respond_to,
        })
        .await
    }
}

impl Handle {
    async fn call<R>(&self, message_fn: impl FnOnce(oneshot::Sender<R>) -> Message) -> R {
        let (tx, rx) = oneshot::channel();
        let message = message_fn(tx);
        tracing::debug!("calling {:?} on a controller named {}", message, self.name);
        self.sender.send(message).await.unwrap();
        rx.await.unwrap()
    }
}

pub struct Master {
    receiver: mpsc::Receiver<Message>,
    slave_providers: Vec<Handle>,
}

impl<'s> Master {
    pub fn new(receiver: mpsc::Receiver<Message>) -> Self {
        Self {
            receiver,
            slave_providers: vec![],
        }
    }

    pub fn insert(&mut self, handle: Handle) {
        self.slave_providers.push(handle);
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await?;
        }
        Ok(())
    }

    async fn handle_message(&mut self, message: Message) -> Result<(), Error> {
        match message {
            Message::WriteCharacteristic {
                accessory_id,
                service_name,
                characteristic,
                respond_to,
            } => {
                let futures = self.slave_providers.iter().map(|provider| async move {
                    (provider, provider.is_connected(accessory_id).await)
                });
                let results = futures::future::join_all(futures).await;
                let provider = results
                    .iter()
                    .find_map(
                        |(provider, is_connected)| {
                            if *is_connected {
                                Some(provider)
                            } else {
                                None
                            }
                        },
                    )
                    .unwrap();

                let result = provider
                    .write_characteristic(accessory_id, service_name, characteristic)
                    .await;
                respond_to.send(result).unwrap();
            }
            Message::ReadCharacteristic {
                accessory_id,
                service_name,
                characteristic_name,
                respond_to,
            } => {
                let futures = self.slave_providers.iter().map(|provider| async move {
                    (provider, provider.is_connected(accessory_id).await)
                });
                let results = futures::future::join_all(futures).await;
                let provider = results
                    .iter()
                    .find_map(
                        |(provider, is_connected)| {
                            if *is_connected {
                                Some(provider)
                            } else {
                                None
                            }
                        },
                    )
                    .unwrap();

                let result = provider
                    .read_characteristic(accessory_id, service_name, characteristic_name)
                    .await;
                respond_to.send(result).unwrap();
            }
            Message::IsConnected {
                accessory_id,
                respond_to,
            } => {
                let futures = self
                    .slave_providers
                    .iter()
                    .map(|provider| provider.is_connected(accessory_id));
                let results: Vec<_> = futures::future::join_all(futures).await;
                let is_connected = results.iter().any(|v| *v);
                respond_to.send(is_connected).unwrap();
            }
            Message::GetAccessories { respond_to } => {
                let futures = self
                    .slave_providers
                    .iter()
                    .map(|provider| async move { provider.get_accessories().await });
                let results: Vec<_> = futures::future::join_all(futures).await;
                let results = results.iter().flat_map(Clone::clone).collect::<Vec<_>>();
                respond_to.send(results).unwrap();
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
