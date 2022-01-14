mod hap;

use std::pin::Pin;

pub use self::hap::HapController as Hap;

use anyhow::Error;
use futures::{Future, FutureExt};
use houseflow_config::hub::Accessory;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::services::ServiceName;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct ControllerHandle {
    pub name: &'static str,
    sender: mpsc::Sender<ActorMessage>,
}

impl ControllerHandle {
    pub fn new(name: &'static str, sender: mpsc::Sender<ActorMessage>) -> Self {
        Self { name, sender }
    }

    pub async fn wait_for_stop(&self) {
        self.sender.closed().await;
    }

    pub async fn connected(&self, configured_accessory: Accessory) {
        self.notify(|| ActorMessage::Connected {
            configured_accessory,
        })
        .await
    }

    pub async fn disconnected(&self, accessory_id: accessory::ID) {
        self.notify(|| ActorMessage::Disconnected { accessory_id })
            .await
    }

    pub async fn updated(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) {
        self.notify(|| ActorMessage::Updated {
            accessory_id,
            service_name,
            characteristic,
        })
        .await
    }
}

impl ControllerHandle {
    async fn notify(&self, message_fn: impl FnOnce() -> ActorMessage) {
        let message = message_fn();
        tracing::debug!("notify {:?} on a controller named {}", message, self.name);
        self.sender.send(message).await.unwrap();
    }
}

#[derive(Debug)]
pub enum ActorMessage {
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

pub struct Master {
    receiver: mpsc::Receiver<ActorMessage>,
    slave_controllers: Vec<ControllerHandle>,
}

impl<'s> Master {
    pub fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
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
        f: impl Fn(&'s ControllerHandle) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>
            + 'a,
    ) -> Result<(), Error> {
        use futures::stream::FuturesOrdered;
        use futures::StreamExt;

        let (controller_names, futures): (Vec<&'static str>, FuturesOrdered<_>) = self
            .slave_controllers
            .iter()
            .map(|controller| (controller.name, f(controller)))
            .unzip();

        let results: Vec<Result<(), Error>> = futures.collect().await;
        for (result, controller) in results.iter().zip(controller_names.iter()) {
            match result {
                Ok(_) => tracing::debug!(controller, "task completed"),
                Err(err) => tracing::error!(controller, "task failed due to {}", err),
            };
        }
        Ok(())
    }

    async fn handle_message(&mut self, message: ActorMessage) -> Result<(), Error> {
        match message {
            ActorMessage::Connected {
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
            ActorMessage::Disconnected { accessory_id } => {
                self.execute_for_all(|controller| {
                    async move {
                        controller.disconnected(accessory_id).await;
                        Ok(())
                    }
                    .boxed()
                })
                .await?;
            }
            ActorMessage::Updated {
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

    pub fn insert(&mut self, handle: ControllerHandle) {
        self.slave_controllers.push(handle);
    }
}
