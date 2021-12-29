pub mod providers;
pub mod services;

use providers::Provider;
use services::Service;

pub struct Hub<S: Service, P: Provider> {
    service: S,
    provider: P,
}

impl<S: Service + 'static, P: Provider + 'static> Hub<S, P> {
    pub async fn new(service: S, provider: P) -> Result<Self, anyhow::Error> {
        Ok(Self { service, provider })
    }

    pub async fn run(
        self,
        provider_events: providers::EventReceiver,
        service_events: services::EventReceiver,
    ) -> Result<(), anyhow::Error> {
        tokio::select! {
            v = self.service.run() => v?,
            v = self.provider.run() => v?,
            v = self.read_provider_events(provider_events) => v?,
            v = self.read_service_events(service_events) => v?,
        }

        Ok(())
    }

    async fn read_provider_events(
        &self,
        mut provider_events: providers::EventReceiver,
    ) -> Result<(), anyhow::Error> {
        while let Some(event) = provider_events.recv().await {
            match event {
                providers::Event::Connected { accessory } => {
                    self.service.connected(&accessory).await?;
                }
                providers::Event::Disconnected { accessory_id } => {
                    self.service.disconnected(&accessory_id).await?
                }
                providers::Event::State {
                    accessory_id,
                    state,
                } => {
                    self.service.update_state(&accessory_id, &state).await?;
                }
            }
        }
        Ok(())
    }

    async fn read_service_events(
        &self,
        mut service_events: services::EventReceiver,
    ) -> Result<(), anyhow::Error> {
        while let Some(event) = service_events.recv().await {
            match event {
                services::Event::Execute(accessory, command) => {
                    let (status, state) = self.provider.execute(accessory, command).await?;
                    tracing::info!("executed on {} with status {} and state {:?}", accessory, status, state);
                    self.service.update_state(&accessory, &state).await?;
                }
            }
        }
        Ok(())
    }
}
