mod hap;

pub use self::hap::HapConfig;
pub use self::hap::HapController;

use anyhow::Error;
use async_trait::async_trait;
use futures::Future;
use futures::FutureExt;
use houseflow_config::hub::Accessory;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::accessory::characteristics::CharacteristicName;
use std::pin::Pin;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Event {
    WriteCharacteristic {
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    },
    ReadCharacteristic {
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    },
}

pub type EventReceiver = mpsc::UnboundedReceiver<Event>;
pub type EventSender = mpsc::UnboundedSender<Event>;

#[async_trait]
pub trait Controller: Send + Sync {
    async fn run(&self) -> Result<(), Error>;
    async fn connected(&self, configured_accessory: &Accessory) -> Result<(), Error>;
    async fn update(
        &self,
        accessory_id: &accessory::ID,
        service_name: &accessory::services::ServiceName,
        characteristic: &accessory::characteristics::Characteristic,
    ) -> Result<(), Error>;
    async fn disconnected(&self, id: &accessory::ID) -> Result<(), Error>;
    fn name(&self) -> &'static str;
}

pub struct MasterController {
    slave_controllers: Vec<Box<dyn Controller + Send + Sync>>,
}

impl<'s> MasterController {
    pub fn new(slave_controllers: Vec<Box<dyn Controller + Send + Sync>>) -> Self {
        Self { slave_controllers }
    }

    async fn execute_for_all<'a>(
        &'s self,
        f: impl Fn(&'s dyn Controller) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>
            + 'a,
    ) -> Result<(), Error> {
        use futures::stream::FuturesOrdered;
        use futures::StreamExt;

        let (controller_names, futures): (Vec<_>, FuturesOrdered<_>) = self
            .slave_controllers
            .iter()
            .map(|controller| (controller.name(), f(controller.as_ref())))
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
}

#[async_trait]
impl Controller for MasterController {
    async fn run(&self) -> Result<(), Error> {
        self.execute_for_all(|controller| {
            async move {
                tracing::info!("starting controller `{}`", controller.name());
                controller.run().await
            }
            .boxed()
        })
        .await?;
        Ok(())
    }

    async fn connected(&self, configured_accessory: &Accessory) -> Result<(), Error> {
        self.execute_for_all(move |controller| controller.connected(configured_accessory))
            .await
    }

    async fn update(
        &self,
        accessory_id: &accessory::ID,
        service_name: &ServiceName,
        characteristic: &Characteristic,
    ) -> Result<(), Error> {
        self.execute_for_all(move |controller| controller.update(accessory_id, service_name, characteristic))
            .await
    }

    async fn disconnected(&self, id: &accessory::ID) -> Result<(), Error> {
        self.execute_for_all(move |controller| controller.disconnected(id))
            .await
    }

    fn name(&self) -> &'static str {
        "master"
    }
}
