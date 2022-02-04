mod hap;
mod lighthouse;

use std::pin::Pin;

pub use self::hap::HapController as Hap;
pub use self::lighthouse::LighthouseController as Lighthouse;

use anyhow::Error;
use futures::{Future, FutureExt};
use houseflow_config::hub::Accessory;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::services::ServiceName;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::IntoStaticStr)]
pub enum Name {
    Master,
    Hap,
    Lighthouse,
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
    sender: acu::Sender<Message>,
}

impl Handle {
    pub fn new(name: Name, sender: acu::Sender<Message>) -> Self {
        Self { name, sender }
    }

    pub async fn wait_for_stop(&self) {
        self.sender.closed().await;
    }

    pub async fn connected(&self, configured_accessory: Accessory) {
        self.sender.notify(|| Message::Connected {
            configured_accessory,
        })
        .await
    }

    pub async fn disconnected(&self, accessory_id: accessory::ID) {
        self.sender.notify(|| Message::Disconnected { accessory_id })
            .await
    }

    pub async fn updated(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) {
        self.sender.notify(|| Message::Updated {
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

    pub async fn run(&mut self) -> Result<(), Error> {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await?;
        }
        Ok(())
    }

    async fn execute_for_all<'a>(
        &'s self,
        f: impl Fn(&'s Handle) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>
            + 'a,
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
                Ok(_) => tracing::debug!(%controller, "task completed"),
                Err(err) => tracing::error!(%controller, "task failed due to {}", err),
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

    pub fn insert(&mut self, handle: Handle) {
        self.slave_controllers.push(handle);
    }
}
