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
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct ControllerHandle {
    pub name: &'static str,
    sender: mpsc::Sender<Message>,
}

impl ControllerHandle {
    pub fn new(name: &'static str, sender: mpsc::Sender<Message>) -> Self {
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
        self.notify(|| Message::Disconnected { accessory_id })
            .await
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

impl ControllerHandle {
    async fn notify(&self, message_fn: impl FnOnce() -> Message) {
        let message = message_fn();
        tracing::debug!("notify {:?} on a controller named {}", message, self.name);
        self.sender.send(message).await.unwrap();
    }
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

pub struct Master {
    receiver: mpsc::Receiver<Message>,
    slave_controllers: Vec<ControllerHandle>,
}

impl<'s> Master {
    pub fn new(receiver: mpsc::Receiver<Message>) -> Self {
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

    pub fn insert(&mut self, handle: ControllerHandle) {
        self.slave_controllers.push(handle);
    }
}
