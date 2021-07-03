use super::{Error, HouseflowAPI};
use houseflow_types::{fulfillment, token::AccessToken};
use reqwest::Client;

#[derive(Debug, thiserror::Error)]
pub enum FulfillmentError {}

impl HouseflowAPI {
    pub async fn sync(&self, access_token: &AccessToken) -> Result<fulfillment::sync::Response, Error> {
        let client = Client::new();
        let url = self.fulfillment_url.join("sync").unwrap();
        let response = client
            .get(url)
            .json(&fulfillment::sync::Request {})
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<fulfillment::sync::Response>()
            .await?;

        Ok(response)
    }

    pub async fn execute(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::execute::Request,
    ) -> Result<fulfillment::execute::Response, Error> {
        let client = Client::new();
        let url = self.fulfillment_url.join("execute").unwrap();
        let response = client
            .post(url)
            .json(request)
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<fulfillment::execute::Response>()
            .await?;

        Ok(response)
    }

    pub async fn query(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::query::Request,
    ) -> Result<fulfillment::query::Response, Error> {
        let client = Client::new();
        let url = self.fulfillment_url.join("query").unwrap();
        let response = client
            .post(url)
            .json(request)
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<fulfillment::query::Response>()
            .await?;

        Ok(response)
    }
}
