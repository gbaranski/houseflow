use super::{Error, HouseflowAPI};
use houseflow_types::admin;
use houseflow_types::token::AccessToken;
use reqwest::Client;

#[derive(Debug, thiserror::Error)]
pub enum AdminError {}

impl HouseflowAPI {
    async fn admin_add_thing<REQ: serde::ser::Serialize, RESP: serde::de::DeserializeOwned>(
        &self,
        access_token: &AccessToken,
        request: &REQ,
        path: &str,
    ) -> Result<RESP, Error> {
        let client = Client::new();

        let url = self.admin_url.join(path).unwrap();
        let response = client
            .put(url)
            .json(request)
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<RESP>()
            .await?;

        Ok(response)
    }

    pub async fn admin_add_device(
        &self,
        access_token: &AccessToken,
        request: &admin::device::add::Request,
    ) -> Result<admin::device::add::Response, Error> {
        self.admin_add_thing(access_token, request, "device").await
    }

    pub async fn admin_add_structure(
        &self,
        access_token: &AccessToken,
        request: &admin::device::add::Request,
    ) -> Result<admin::structure::add::Response, Error> {
        self.admin_add_thing(access_token, request, "structure")
            .await
    }

    pub async fn admin_add_room(
        &self,
        access_token: &AccessToken,
        request: &admin::device::add::Request,
    ) -> Result<admin::room::add::Response, Error> {
        self.admin_add_thing(access_token, request, "room").await
    }

    pub async fn admin_add_user_structure(
        &self,
        access_token: &AccessToken,
        request: &admin::user_structure::add::Request,
    ) -> Result<admin::user_structure::add::Response, Error> {
        self.admin_add_thing(access_token, request, "user_structure")
            .await
    }
}
