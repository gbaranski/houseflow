mod hive;
mod mijia;

pub use self::hive::HiveProvider as Hive;
pub use self::mijia::MijiaProvider as Mijia;

use anyhow::Error;
use houseflow_config::hub::Accessory;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum ProviderName {
    Master,
    Hive,
    Mijia,
}

#[derive(Debug, Clone)]
pub struct ProviderHandle {
    pub name: ProviderName,
    sender: mpsc::Sender<ProviderMessage>,
}

impl ProviderHandle {
    pub fn new(name: ProviderName, sender: mpsc::Sender<ProviderMessage>) -> Self {
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
        self.call(|respond_to| ProviderMessage::WriteCharacteristic {
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
        self.call(|respond_to| ProviderMessage::ReadCharacteristic {
            accessory_id,
            service_name,
            characteristic_name,
            respond_to,
        })
        .await
    }

    pub async fn is_connected(&self, accessory_id: accessory::ID) -> bool {
        self.call(|respond_to| ProviderMessage::IsConnected {
            accessory_id,
            respond_to,
        })
        .await
    }
    pub async fn get_accessory_configuration(
        &self,
        accessory_id: accessory::ID,
    ) -> Option<Accessory> {
        self.call(|respond_to| ProviderMessage::GetAccessoryConfiguration {
            accessory_id,
            respond_to,
        })
        .await
    }
}

impl ProviderHandle {
    async fn call<R>(&self, message_fn: impl FnOnce(oneshot::Sender<R>) -> ProviderMessage) -> R {
        let (tx, rx) = oneshot::channel();
        let message = message_fn(tx);
        tracing::debug!("calling {:?} on a controller named {}", message, self.name);
        self.sender.send(message).await.unwrap();
        rx.await.unwrap()
    }
}

#[derive(Debug)]
pub enum ProviderMessage {
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
    GetAccessoryConfiguration {
        accessory_id: accessory::ID,
        respond_to: oneshot::Sender<Option<Accessory>>,
    },
    IsConnected {
        accessory_id: accessory::ID,
        respond_to: oneshot::Sender<bool>,
    },
}

pub struct Master {
    receiver: mpsc::Receiver<ProviderMessage>,
    slave_providers: Vec<ProviderHandle>,
}

impl<'s> Master {
    pub fn new(receiver: mpsc::Receiver<ProviderMessage>) -> Self {
        Self {
            receiver,
            slave_providers: vec![],
        }
    }

    pub fn insert(&mut self, handle: ProviderHandle) {
        self.slave_providers.push(handle);
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await?;
        }
        Ok(())
    }

    async fn handle_message(&mut self, message: ProviderMessage) -> Result<(), Error> {
        match message {
            ProviderMessage::WriteCharacteristic {
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
            ProviderMessage::ReadCharacteristic {
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
            ProviderMessage::IsConnected {
                accessory_id,
                respond_to,
            } => {
                let futures = self
                    .slave_providers
                    .iter()
                    .map(|provider| provider.is_connected(accessory_id));
                let results: Vec<_> = futures::future::join_all(futures).await;
                let is_connected = results.iter().any(|v| *v == true);
                respond_to.send(is_connected).unwrap();
            }
            ProviderMessage::GetAccessoryConfiguration {
                accessory_id,
                respond_to,
            } => {
                let provider = self.slave_providers.iter().next().unwrap(); // TODO: Do something else there
                let accessory_configuration =
                    provider.get_accessory_configuration(accessory_id).await;
                respond_to.send(accessory_configuration).unwrap();
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
