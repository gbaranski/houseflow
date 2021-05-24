use houseflow_auth_types::{AccessTokenRequest, AccessTokenResponse, GrantType};
use houseflow_token::Token;
use reqwest::Client;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use url::Url;

#[derive(Clone)]
pub struct Auth {
    url: Url,
    refresh_token: Token,
    access_token: Arc<Mutex<Token>>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("error occured with sending request: `{0}`")]
    ReqwestError(#[from] reqwest::Error),

    #[error("refreshing access token failed with: `{0}`")]
    RefreshAccessTokenError(#[from] houseflow_auth_types::AccessTokenRequestError),
}

impl Auth {
    pub async fn new(url: Url, refresh_token: Token) -> Result<Self, Error> {
        let access_token = Self::fetch_access_token(&url, &refresh_token).await?;

        Ok(Self {
            url,
            refresh_token,
            access_token: Arc::new(Mutex::new(access_token)),
        })
    }

    pub async fn access_token(&self) -> Result<Token, Error> {
        if !self.access_token.lock().await.has_expired() {
            return Ok(self.access_token.lock().await.clone());
        }
        let access_token = Self::fetch_access_token(&self.url, &self.refresh_token).await?;
        *self.access_token.lock().await = access_token.clone();

        Ok(access_token)
    }

    async fn fetch_access_token(url: &Url, refresh_token: &Token) -> Result<Token, Error> {
        let client = Client::new();
        let request = AccessTokenRequest {
            grant_type: GrantType::RefreshToken,
            refresh_token: refresh_token.clone(),
        };
        let url = url.join("token").unwrap();

        let response = client
            .post(url)
            .query(&request)
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await?;
        Ok(response.access_token)
    }
}
