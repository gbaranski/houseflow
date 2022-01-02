pub mod controllers;
pub mod providers;

use controllers::Controller;
use providers::Provider;

pub struct Hub<C: Controller, P: Provider> {
    controller: C,
    provider: P,
}

impl<C: Controller + 'static, P: Provider + 'static> Hub<C, P> {
    pub async fn new(controller: C, provider: P) -> Result<Self, anyhow::Error> {
        Ok(Self {
            controller,
            provider,
        })
    }

    pub async fn run(
        self,
        provider_events: providers::EventReceiver,
        controller_events: controllers::EventReceiver,
    ) -> Result<(), anyhow::Error> {
        tokio::select! {
            v = self.controller.run() => v?,
            v = self.provider.run() => v?,
            v = self.read_provider_events(provider_events) => v?,
            v = self.read_controller_events(controller_events) => v?,
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
                    self.controller.connected(&accessory).await?;
                }
                providers::Event::Disconnected { accessory_id } => {
                    self.controller.disconnected(&accessory_id).await?
                }
                providers::Event::CharacteristicUpdate {
                    accessory_id,
                    service_name,
                    characteristic,
                } => {
                    self.controller
                        .update(&accessory_id, &service_name, &characteristic)
                        .await?
                }
            }
        }
        Ok(())
    }

    async fn read_controller_events(
        &self,
        mut controller_events: controllers::EventReceiver,
    ) -> Result<(), anyhow::Error> {
        while let Some(event) = controller_events.recv().await {
            match event {
                controllers::Event::WriteCharacteristic {
                    accessory_id,
                    service_name,
                    characteristic,
                } => {
                    self.provider
                        .write_characteristic(&accessory_id, &service_name, &characteristic)
                        .await?
                        .map_err(|err| {
                            anyhow::anyhow!("writing characteristic failed with {}", err)
                        })?;
                    tracing::info!(
                        %accessory_id,
                        %service_name,
                        ?characteristic,
                        "wrote characteristic"
                    );
                    self.controller
                        .update(&accessory_id, &service_name, &characteristic)
                        .await?;
                }
                controllers::Event::ReadCharacteristic {
                    accessory_id,
                    service_name,
                    characteristic_name,
                } => {
                    let characteristic = self
                        .provider
                        .read_characteristic(&accessory_id, &service_name, &characteristic_name)
                        .await?
                        .unwrap();
                    tracing::info!(
                        %accessory_id,
                        %service_name,
                        %characteristic_name,
                        "read characteristic = {:?}", characteristic
                    );
                    self.controller
                        .update(&accessory_id, &service_name, &characteristic)
                        .await?;
                }
            }
        }
        Ok(())
    }
}
