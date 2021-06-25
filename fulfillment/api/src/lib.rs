use fulfillment_types::{ExecuteRequest, ExecuteResponse, SyncRequest, SyncResponse};
use reqwest::Client;
use token::Token;
use url::Url;

#[derive(Clone)]
pub struct Fulfillment {
    pub url: Url,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Sending request failed: `{0}`")]
    ReqwestError(#[from] reqwest::Error),
}

impl Fulfillment {
    pub fn new(server_address: std::net::SocketAddr) -> Self {
        Self {
            url: Url::parse(&format!("http://{}/fulfillment/internal/", server_address)).unwrap(),
        }
    }

    pub async fn sync(&self, access_token: &Token) -> Result<SyncResponse, Error> {
        let client = Client::new();
        let url = self.url.join("sync").unwrap();
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
        let url = self.url.join("execute").unwrap();
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
