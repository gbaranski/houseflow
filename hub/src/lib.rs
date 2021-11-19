pub mod services;
pub mod providers;

use providers::Provider;
use services::Service;

pub struct Hub<S: Service, P: Provider> {
    service: S,
    provider: P,
}

impl<S: Service, P: Provider> Hub<S, P> {
    pub async fn new(service: S, provider: P) -> Result<Self, anyhow::Error> {
        Ok(Self {  service, provider })
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        let service_fut = self.service.run();
        let provider_fut = self.provider.run();
        let (service_result, provider_result) = tokio::join!(service_fut, provider_fut);
        service_result?;
        provider_result?;
        Ok(())
    }
}
