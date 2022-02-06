pub mod controllers;
pub mod providers;

pub struct Hub {
    controller: controllers::Handle,
    provider: providers::Handle,
}

impl Hub {
    pub async fn new(
        controller: controllers::Handle,
        provider: providers::Handle,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            controller,
            provider,
        })
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        tokio::select! {
            _ = self.controller.wait_for_stop() => {
                tracing::info!("controller {} has stopped", self.controller.name());
            },
            _ = self.provider.wait_for_stop() => {
                tracing::info!("provider {} has stopped", self.provider.name());
            },
        }

        Ok(())
    }
}
