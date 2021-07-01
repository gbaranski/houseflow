use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use clap::Clap;

use houseflow_types::{admin::AddUserStructureRequest, StructureID, UserID};

#[derive(Clap)]
pub struct AddUserStructureCommand {
    /// ID of the structure
    structure_id: StructureID,

    /// ID of the user
    user_id: UserID,

    /// True if user is manager in relation to the structure
    #[clap(parse(try_from_str))]
    is_manager: bool,
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for AddUserStructureCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        let request = AddUserStructureRequest {
            structure_id: self.structure_id,
            user_id: self.user_id,
            is_manager: self.is_manager,
        };

        let access_token = state.access_token().await?;
        state
            .houseflow_api
            .admin_add_user_structure(&access_token, &request)
            .await?
            .into_result()?;

        log::info!("✔ Succesfully added user structure");

        Ok(())
    }
}
