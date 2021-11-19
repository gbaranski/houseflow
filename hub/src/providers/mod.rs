mod mijia;

pub use self::mijia::MijiaProvider;

use std::pin::Pin;
use anyhow::Error;
use async_trait::async_trait;
use futures::Future;

pub struct Accessory {}

#[async_trait]
pub trait Provider {
    async fn run(&self) -> Result<(), Error>;
    async fn discover(&self) -> Result<Option<Accessory>, Error>;
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
        f: impl Fn(&'s dyn Provider) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> + 'a,
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
        todo!()
    }

    async fn discover(&self) -> Result<Option<Accessory>, Error> {
        todo!()
    }

    fn name(&self) -> &'static str {
        "master"
    }
}
