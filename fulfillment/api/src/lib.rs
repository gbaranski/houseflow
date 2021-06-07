use auth_api::Auth;
use token::Token;
use fulfillment_types::{SyncRequest, SyncResponse};
use types::Device;
use reqwest::Client;
use url::Url;

pub struct Fulfillment {
    url: Url,
    auth: Auth,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Auth API Error: `{0}`")]
    AuthError(#[from] auth_api::Error),

    #[error("Sending request failed: `{0}`")]
    ReqwestError(#[from] reqwest::Error),
}

impl Fulfillment {
    pub fn new(url: Url, auth: Auth) -> Self {
        Self { url, auth }
    }

    pub async fn sync(&self) -> Result<Vec<Device>, Error> {
        let access_token: Token = unimplemented!();
        let client = Client::new();
        let url = self.url.join("sync").unwrap();
        let response = client
            .post(url)
            .json(&SyncRequest::default())
            .bearer_auth(access_token.to_string())
            .send()
            .await?
            .json::<SyncResponse>()
            .await?;

        Ok(response)
    }
}
