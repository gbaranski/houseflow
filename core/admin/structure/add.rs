use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use clap::Clap;

use houseflow_types::admin::AddStructureRequest;

#[derive(Clap)]
pub struct AddStructureCommand {
    /// Name of the structure
    name: String,
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for AddStructureCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        // TODO: try to simplify that
        let request = AddStructureRequest {
            structure_name: self.name,
        };

        let access_token = state.access_token().await?;
        let response = state
            .houseflow_api
            .admin_add_structure(&access_token, &request)
            .await?
            .into_result()?;

        log::info!("✔ Succesfully added structure with ID: {}", response.structure_id);

        Ok(())
    }
}
