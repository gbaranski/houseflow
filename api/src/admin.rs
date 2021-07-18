use crate::{put_with_token, Error, HouseflowAPI};
use houseflow_types::admin;
use houseflow_types::token::AccessToken;

#[derive(Debug, thiserror::Error)]
pub enum AdminError {}

#[macro_use]
macro_rules! admin_method {
    ($name: ident, $path: literal, $($tmod: ident)::+) => {
        pub async fn $name(
            &self,
            access_token: &AccessToken,
            request: &$($tmod)::+::add::Request,
        ) -> Result<$($tmod)::+::add::Response, Error> {
            let url = self.admin_url.join($path).unwrap();
            put_with_token(url, request, access_token).await
        }
    };
}

impl HouseflowAPI {
    admin_method!(admin_add_structure, "structure", admin::structure);
    admin_method!(admin_add_room, "room", admin::room);
    admin_method!(admin_add_device, "device", admin::device);
    admin_method!(
        admin_add_user_structure,
        "user_structure",
        admin::user_structure
    );
}
