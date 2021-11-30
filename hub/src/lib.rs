pub mod providers;
pub mod services;

use providers::Provider;
use services::Service;

use crate::services::AdditionalAccessoryInfo;

#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct AccessoryState {
    pub temperature: Option<f32>,
    pub humidity: Option<u8>,
    pub battery_percent: Option<u16>,
    pub battery_voltage: Option<u16>,
}

pub struct Hub<S: Service, P: Provider> {
    service: S,
    provider: P,
    provider_events: providers::EventReceiver,
}

impl<S: Service, P: Provider> Hub<S, P> {
    pub async fn new(
        service: S,
        provider: P,
        provider_events: providers::EventReceiver,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            service,
            provider,
            provider_events,
        })
    }

    pub async fn run(mut self) -> Result<(), anyhow::Error> {
        let service_fut = self.service.run();
        let provider_fut = self.provider.run();
        let provider_events_fut = async {
            while let Some(event) = self.provider_events.recv().await {
                match event {
                    providers::Event::Connected(configured_accessory) => {
                        self.service
                            .connected(&configured_accessory, &AdditionalAccessoryInfo {})
                            .await?;
                    }
                    providers::Event::StateUpdate(id, state) => {
                        self.service.update_state(&id, &state).await?;
                    }
                }
            }
            Ok::<(), anyhow::Error>(())
        };
        let (service_result, provider_result, provider_events_result) =
            tokio::join!(service_fut, provider_fut, provider_events_fut);
        service_result?;
        provider_result?;
        provider_events_result?;
        Ok(())
    }
}
