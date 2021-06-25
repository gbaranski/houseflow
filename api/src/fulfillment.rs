use super::{Error, HouseflowAPI};
use fulfillment_types::{ExecuteRequest, ExecuteResponse, SyncRequest, SyncResponse};
use reqwest::Client;
use token::Token;

#[derive(Debug, thiserror::Error)]
pub enum FulfillmentError {}

impl HouseflowAPI {
    pub async fn sync(&self, access_token: &Token) -> Result<SyncResponse, Error> {
        let client = Client::new();
        let url = self.fulfillment_url.join("sync").unwrap();
        let response = client
            .get(url)
            .json(&SyncRequest {})
            .bearer_auth(access_token.to_string())
            .send()
            .await?
            .json::<SyncResponse>()
            .await?;

        Ok(response)
    }

    pub async fn execute(
        &self,
        access_token: &Token,
        request: &ExecuteRequest,
    ) -> Result<ExecuteResponse, Error> {
        let client = Client::new();
        let url = self.fulfillment_url.join("execute").unwrap();
        let response = client
            .post(url)
            .json(request)
            .bearer_auth(access_token.to_string())
            .send()
            .await?
            .json::<ExecuteResponse>()
            .await?;

        Ok(response)
    }
}
