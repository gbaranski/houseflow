use super::{Error, HouseflowAPI};
use houseflow_types::fulfillment::{
    ExecuteRequest, ExecuteResponse, QueryRequest, QueryResponse, SyncRequest, SyncResponse,
};
use houseflow_types::token::Token;
use reqwest::Client;

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

    pub async fn query(
        &self,
        access_token: &Token,
        request: &QueryRequest,
    ) -> Result<QueryResponse, Error> {
        let client = Client::new();
        let url = self.fulfillment_url.join("query").unwrap();
        let response = client
            .post(url)
            .json(request)
            .bearer_auth(access_token.to_string())
            .send()
            .await?
            .json::<QueryResponse>()
            .await?;

        Ok(response)
    }
}
