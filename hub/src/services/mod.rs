mod hap;

pub use self::hap::HapConfig;
pub use self::hap::HapService;

use crate::AccessoryState;
use anyhow::Error;
use async_trait::async_trait;
use futures::Future;
use houseflow_config::hub::Accessory;
use houseflow_types::accessory;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct AdditionalAccessoryInfo {}

#[async_trait]
pub trait Service {
    async fn run(&self) -> Result<(), Error>;
    async fn connected(
        &self,
        configured_accessory: &Accessory,
        additional_accessory_info: &AdditionalAccessoryInfo,
    ) -> Result<(), Error>;
    async fn update_state(&self, id: &accessory::ID, state: &AccessoryState) -> Result<(), Error>;
    async fn disconnected(&self, id: &accessory::ID) -> Result<(), Error>;
    fn name(&self) -> &'static str;
}

pub struct MasterService {
    slave_services: Vec<Box<dyn Service + Send + Sync>>,
}

impl<'s> MasterService {
    pub fn new(slave_services: Vec<Box<dyn Service + Send + Sync>>) -> Self {
        Self { slave_services }
    }

    async fn execute_for_all<'a>(
        &'s self,
        f: impl Fn(&'s dyn Service) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> + 'a,
    ) -> Result<(), Error> {
        use futures::stream::FuturesOrdered;
        use futures::StreamExt;

        let (service_names, futures): (Vec<_>, FuturesOrdered<_>) = self
            .slave_services
            .iter()
            .map(|service| (service.name(), f(service.as_ref())))
            .unzip();
        let results: Vec<Result<(), Error>> = futures.collect().await;
        for (result, service) in results.iter().zip(service_names.iter()) {
            match result {
                Ok(_) => tracing::debug!(service, "task completed"),
                Err(err) => tracing::error!(service, "task failed due to {}", err),
            };
        }
        Ok(())
    }
}

#[async_trait]
impl Service for MasterService {
    async fn run(&self) -> Result<(), Error> {
        self.execute_for_all(|service| service.run()).await?;
        Ok(())
    }

    async fn connected(
        &self,
        configured_accessory: &Accessory,
        additional_accessory_info: &AdditionalAccessoryInfo,
    ) -> Result<(), Error> {
        self.execute_for_all(move |service| {
            service.connected(configured_accessory, additional_accessory_info)
        })
        .await
    }

    async fn update_state(&self, id: &accessory::ID, state: &AccessoryState) -> Result<(), Error> {
        self.execute_for_all(move |service| service.update_state(id, state))
            .await
    }

    async fn disconnected(&self, id: &accessory::ID) -> Result<(), Error> {
        self.execute_for_all(move |service| service.disconnected(id))
            .await
    }

    fn name(&self) -> &'static str {
        "master"
    }
}
