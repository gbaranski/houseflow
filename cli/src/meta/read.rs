use crate::CommandContext;
use async_trait::async_trait;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;

pub struct Command {
    pub accessory_id: accessory::ID,
    pub service_name: ServiceName,
    pub characteristic_name: CharacteristicName,
}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        let characteristic = ctx
            .server_client()?
            .read_characteristics(
                &self.accessory_id,
                &self.service_name,
                &self.characteristic_name,
            )
            .await??;
        tracing::info!("Characteristic: {:?}", characteristic);
        Ok(())
    }
}
