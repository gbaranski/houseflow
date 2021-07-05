use crate::{post_with_token, Error, HouseflowAPI};
use houseflow_types::admin;
use houseflow_types::token::AccessToken;

#[derive(Debug, thiserror::Error)]
pub enum AdminError {}

impl HouseflowAPI {
    pub async fn admin_add_device(
        &self,
        access_token: &AccessToken,
        request: &admin::device::add::Request,
    ) -> Result<admin::device::add::Response, Error> {
        let url = self.admin_url.join("device").unwrap();
        post_with_token(url, request, access_token).await
    }

    pub async fn admin_add_structure(
        &self,
        access_token: &AccessToken,
        request: &admin::structure::add::Request,
    ) -> Result<admin::structure::add::Response, Error> {
        let url = self.admin_url.join("structure").unwrap();
        post_with_token(url, request, access_token).await
    }

    pub async fn admin_add_room(
        &self,
        access_token: &AccessToken,
        request: &admin::room::add::Request,
    ) -> Result<admin::room::add::Response, Error> {
        let url = self.admin_url.join("room").unwrap();
        post_with_token(url, request, access_token).await
    }

    pub async fn admin_add_user_structure(
        &self,
        access_token: &AccessToken,
        request: &admin::user_structure::add::Request,
    ) -> Result<admin::user_structure::add::Response, Error> {
        let url = self.admin_url.join("user_structure").unwrap();
        post_with_token(url, request, access_token).await
    }
}
