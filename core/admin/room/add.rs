use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use clap::Clap;

use houseflow_types::{admin, StructureID};

#[derive(Clap)]
pub struct AddRoomCommand {
    /// Name of the room
    name: String,

    /// ID of the structure
    structure_id: StructureID,
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for AddRoomCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        let request = admin::room::add::Request {
            room_name: self.name,
            structure_id: self.structure_id,
        };

        let access_token = state.access_token().await?;
        let response = state
            .houseflow_api
            .admin_add_room(&access_token, &request)
            .await??;

        log::info!("✔ Succesfully added room with ID: {}", response.room_id);

        Ok(())
    }
}
