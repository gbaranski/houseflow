#[macro_use]
macro_rules! add_command {
    ($module: ident, $command: ty, $add_method: expr, $callback: expr) => {
        pub mod $module {
            use super::*;

            pub mod add {
                use super::*;
                use crate::CommandContext;
                use async_trait::async_trait;

                use houseflow_types::admin;

                pub type Command = $command;

                #[async_trait]
                impl crate::Command for Command {
                    async fn run(self, ctx: CommandContext) -> anyhow::Result<()> {
                        let access_token = ctx.access_token().await?;
                        let response =
                            $add_method(&ctx.houseflow_api, &access_token, &self).await??;

                        ($callback)(response);

                        Ok(())
                    }
                }
            }
        }
    };
}

use houseflow_api::HouseflowAPI;

add_command!(
    structure,
    admin::structure::add::Request,
    HouseflowAPI::admin_add_structure,
    (|res: admin::structure::add::ResponseBody| {
        tracing::info!(
            "✔ Succesfully added structure with ID: {}",
            res.structure_id
        );
    })
);

add_command!(
    room,
    admin::room::add::Request,
    HouseflowAPI::admin_add_room,
    (|res: admin::room::add::ResponseBody| {
        tracing::info!("✔ Succesfully added room with ID: {}", res.room_id);
    })
);

add_command!(
    device,
    admin::device::add::Request,
    HouseflowAPI::admin_add_device,
    (|res: admin::device::add::ResponseBody| {
        tracing::info!("✔ Succesfully added device with ID: {}", res.device_id);
    })
);

add_command!(
    user_structure,
    admin::user_structure::add::Request,
    HouseflowAPI::admin_add_user_structure,
    (|_: admin::user_structure::add::ResponseBody| {
        tracing::info!("✔ Succesfully added user structure");
    })
);
