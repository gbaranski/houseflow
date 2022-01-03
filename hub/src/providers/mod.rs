mod hive;
mod mijia;

pub use self::hive::HiveProvider;
pub use self::mijia::MijiaProvider;

use anyhow::Error;
use async_trait::async_trait;
use futures::{Future, FutureExt};
use houseflow_config::hub::Accessory;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicDiscriminants;
use houseflow_types::accessory::services::ServiceName;
use std::pin::Pin;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Event {
    Connected {
        accessory: Accessory,
    },
    Disconnected {
        accessory_id: accessory::ID,
    },
    CharacteristicUpdate {
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    },
}

pub type EventReceiver = mpsc::UnboundedReceiver<Event>;
pub type EventSender = mpsc::UnboundedSender<Event>;

#[async_trait]
pub trait Provider: Send + Sync {
    async fn run(&self) -> Result<(), Error>;
    async fn write_characteristic(
        &self,
        accessory_id: &accessory::ID,
        service_name: &ServiceName,
        characteristic: &Characteristic,
    ) -> Result<Result<(), accessory::Error>, Error>;
    async fn read_characteristic(
        &self,
        accessory_id: &accessory::ID,
        service_name: &ServiceName,
        characteristic_name: &CharacteristicDiscriminants,
    ) -> Result<Result<Characteristic, accessory::Error>, Error>;
    async fn is_connected(&self, accessory_id: &accessory::ID) -> bool;
    fn name(&self) -> &'static str;
}

pub struct MasterProvider {
    slave_providers: Vec<Box<dyn Provider>>,
}

impl<'s> MasterProvider {
    pub fn new(slave_providers: Vec<Box<dyn Provider>>) -> Self {
        Self { slave_providers }
    }

    async fn execute_for_all<'a>(
        &'s self,
        f: impl Fn(&'s dyn Provider) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>
            + 'a,
    ) -> Result<(), Error> {
        use futures::stream::FuturesOrdered;
        use futures::StreamExt;

        let (provider_names, futures): (Vec<_>, FuturesOrdered<_>) = self
            .slave_providers
            .iter()
            .map(|provider| (provider.name(), f(provider.as_ref())))
            .unzip();
        let results: Vec<Result<(), Error>> = futures.collect().await;
        for (result, provider) in results.iter().zip(provider_names.iter()) {
            match result {
                Ok(_) => tracing::debug!(provider, "task completed"),
                Err(err) => tracing::error!(provider, "task failed due to {}", err),
            };
        }
        Ok(())
    }
}

#[async_trait]
impl Provider for MasterProvider {
    async fn run(&self) -> Result<(), Error> {
        self.execute_for_all(|provider| {
            async move {
                tracing::info!("starting provider `{}`", provider.name());
                provider.run().await
            }
            .boxed()
        })
        .await
    }

    async fn write_characteristic(
        &self,
        accessory_id: &accessory::ID,
        service_name: &ServiceName,
        characteristic: &Characteristic,
    ) -> Result<Result<(), accessory::Error>, Error> {
        let futures = self
            .slave_providers
            .iter()
            .map(|provider| async move { (provider, provider.is_connected(&accessory_id).await) });
        let results = futures::future::join_all(futures).await;
        let provider = results
            .iter()
            .find_map(|(provider, is_connected)| if *is_connected { Some(provider) } else { None })
            .unwrap();

        provider.write_characteristic(accessory_id, service_name, characteristic).await
    }

    async fn read_characteristic(
        &self,
        accessory_id: &accessory::ID,
        service_name: &ServiceName,
        characteristic_name: &CharacteristicDiscriminants,
    ) -> Result<Result<Characteristic, accessory::Error>, Error> {
        let futures = self
            .slave_providers
            .iter()
            .map(|provider| async move { (provider, provider.is_connected(&accessory_id).await) });
        let results = futures::future::join_all(futures).await;
        let provider = results
            .iter()
            .find_map(|(provider, is_connected)| if *is_connected { Some(provider) } else { None })
            .unwrap();

        provider.read_characteristic(accessory_id, service_name, characteristic_name).await

    }

    async fn is_connected(&self, accessory_id: &accessory::ID) -> bool {
        let futures = self
            .slave_providers
            .iter()
            .map(|provider| provider.is_connected(accessory_id));
        let results: Vec<_> = futures::future::join_all(futures).await;
        results.iter().any(|v| *v == true)
    }

    fn name(&self) -> &'static str {
        "master"
    }
}
