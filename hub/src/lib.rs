pub mod controllers;
pub mod providers;

use controllers::ControllerHandle;
use providers::ProviderHandle;

pub struct Hub {
    controller: ControllerHandle,
    provider: ProviderHandle,
}

impl Hub {
    pub async fn new(
        controller: ControllerHandle,
        provider: ProviderHandle,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            controller,
            provider,
        })
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        tokio::select! {
            _ = self.controller.wait_for_stop() => {
                tracing::info!("controller {} has stopped", self.controller.name);
            },
            _ = self.provider.wait_for_stop() => {
                tracing::info!("provider {} has stopped", self.provider.name);
            },
        }

        Ok(())
    }
}
