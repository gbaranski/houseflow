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
use tokio::sync::oneshot;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::IntoStaticStr)]
pub enum Name {
    Master,
    Hive,
    Mijia,
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

    pub async fn write_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), accessory::Error> {
        self.sender
            .call(|respond_to| Message::WriteCharacteristic {
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
        self.sender
            .call(|respond_to| Message::ReadCharacteristic {
                accessory_id,
                service_name,
                characteristic_name,
                respond_to,
            })
            .await
    }

    pub async fn is_connected(&self, accessory_id: accessory::ID) -> bool {
        self.sender
            .call(|respond_to| Message::IsConnected {
                accessory_id,
                respond_to,
            })
            .await
    }
    pub async fn get_accessory_configuration(
        &self,
        accessory_id: accessory::ID,
    ) -> Option<Accessory> {
        self.sender
            .call(|respond_to| Message::GetAccessoryConfiguration {
                accessory_id,
                respond_to,
            })
            .await
    }
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
    receiver: acu::Receiver<Message>,
    slave_providers: Vec<Handle>,
}

impl<'s> Master {
    pub fn new(receiver: acu::Receiver<Message>) -> Self {
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
            Message::GetAccessoryConfiguration {
                accessory_id,
                respond_to,
            } => {
                let provider = self.slave_providers.get(0).unwrap(); // TODO: Do something else there
                let accessory_configuration =
                    provider.get_accessory_configuration(accessory_id).await;
                respond_to.send(accessory_configuration).unwrap();
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::IntoStaticStr)]
pub enum SessionName {
    HiveSession,
    MijiaSession,
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
