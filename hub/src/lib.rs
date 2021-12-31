pub mod providers;
pub mod controllers;

use providers::Provider;
use controllers::Controller;

pub struct Hub<C: Controller, P: Provider> {
    controller: C,
    provider: P,
}

impl<C: Controller + 'static, P: Provider + 'static> Hub<C, P> {
    pub async fn new(controller: C, provider: P) -> Result<Self, anyhow::Error> {
        Ok(Self { controller, provider })
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
                providers::Event::State {
                    accessory_id,
                    state,
                } => {
                    self.controller.update_state(&accessory_id, &state).await?;
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
                controllers::Event::Execute(accessory, command) => {
                    let (status, state) = self.provider.execute(accessory, command).await?;
                    tracing::info!("executed on {} with status {} and state {:?}", accessory, status, state);
                    self.controller.update_state(&accessory, &state).await?;
                }
            }
        }
        Ok(())
    }
}
