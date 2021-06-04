use houseflow_auth_types::{
    AccessTokenError, AccessTokenRequest, AccessTokenResponse, GrantType, LoginError, LoginRequest,
    LoginResponse, RegisterError, RegisterRequest, RegisterResponse,
};
use houseflow_token::Token;
use reqwest::Client;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use url::Url;

#[derive(Clone)]
pub struct Auth {
    url: Url,
    refresh_token: Arc<Mutex<Option<Token>>>,
    access_token: Arc<Mutex<Option<Token>>>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("error occured with sending request: `{0}`")]
    ReqwestError(#[from] reqwest::Error),

    #[error("refreshing access token failed with: `{0}`")]
    RefreshAccessTokenError(#[from] AccessTokenError),

    #[error("not logged in")]
    NotLoggedIn,

    #[error("registration failed: `{0}`")]
    RegisterError(#[from] RegisterError),

    #[error("login failed: `{0}`")]
    LoginError(#[from] LoginError),
}

impl Auth {
    pub async fn new(url: Url) -> Result<Self, Error> {
        Ok(Self {
            url,
            refresh_token: Default::default(),
            access_token: Default::default(),
        })
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<(), Error> {
        let client = Client::new();
        let url = self.url.join("register").unwrap();

        let _ = client
            .post(url)
            .query(&request)
            .send()
            .await?
            .json::<RegisterResponse>()
            .await??;

        Ok(())
    }

    pub async fn login(&self, request: LoginRequest) -> Result<(), Error> {
        let client = Client::new();
        let url = self.url.join("login").unwrap();

        let response = client
            .post(url)
            .query(&request)
            .send()
            .await?
            .json::<LoginResponse>()
            .await??;
        *self.refresh_token.lock().await = Some(response.refresh_token);
        *self.access_token.lock().await = Some(response.access_token);

        Ok(())
    }

    pub async fn refresh_token(&self) -> Result<Token, Error> {
        let refresh_token = self.refresh_token.lock().await;
        match refresh_token.as_ref() {
            Some(token) => Ok(token.clone()),
            None => Err(Error::NotLoggedIn),
        }
    }

    pub async fn access_token(&self) -> Result<Token, Error> {
        let mut access_token = self.access_token.lock().await;
        let refresh_token = self.refresh_token().await?;

        let access_token = match access_token.as_ref() {
            Some(token) if !token.has_expired() => token.clone(),
            Some(_) | None => {
                let new_access_token = Self::fetch_access_token(&self.url, &refresh_token).await?;
                *access_token = Some(new_access_token.clone());
                new_access_token
            }
        };

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
            .await??;

        Ok(response.access_token)
    }
}
