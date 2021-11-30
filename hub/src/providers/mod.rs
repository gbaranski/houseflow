mod mijia;

pub use self::mijia::MijiaProvider;

use crate::AccessoryState;
use anyhow::Error;
use async_trait::async_trait;
use futures::Future;
use houseflow_types::accessory::ID as AccessoryID;
use std::pin::Pin;
use houseflow_config::hub::Accessory;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Event {
    Connected(Accessory),
    StateUpdate(AccessoryID, AccessoryState),
}

pub type EventReceiver = mpsc::UnboundedReceiver<Event>;
pub type EventSender = mpsc::UnboundedSender<Event>;

#[async_trait]
pub trait Provider {
    async fn run(&self) -> Result<(), Error>;
    fn name(&self) -> &'static str;
}

pub struct MasterProvider {
    slave_providers: Vec<Box<dyn Provider + Send + Sync>>,
}

impl<'s> MasterProvider {
    pub fn new(slave_providers: Vec<Box<dyn Provider + Send + Sync>>) -> Self {
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
        self.execute_for_all(|provider| provider.run()).await
    }

    fn name(&self) -> &'static str {
        "master"
    }
}
