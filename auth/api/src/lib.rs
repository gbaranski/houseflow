use auth_types::{
    AccessTokenRequest, AccessTokenResponse, AccessTokenResponseError, GrantType, LoginRequest,
    LoginResponse, LoginResponseError, LogoutResponse, RegisterRequest, RegisterResponse,
    RegisterResponseError, WhoamiResponse,
};
use reqwest::Client;
use thiserror::Error;
use token::Token;
use url::Url;

#[derive(Clone)]
pub struct Auth {
    url: Url,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("error occured with sending request: `{0}`")]
    ReqwestError(#[from] reqwest::Error),

    #[error("refreshing access token failed with: `{0}`")]
    RefreshAccessTokenError(#[from] AccessTokenResponseError),

    #[error("not logged in")]
    NotLoggedIn,

    #[error("registration failed: `{0}`")]
    RegisterError(#[from] RegisterResponseError),

    #[error("login failed: `{0}`")]
    LoginError(#[from] LoginResponseError),
}

impl Auth {
    pub fn new(auth_url: Url) -> Self {
        Self { url: auth_url }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, Error> {
        let client = Client::new();
        let url = self.url.join("register").unwrap();

        let response = client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<RegisterResponse>()
            .await?;

        Ok(response)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, Error> {
        let client = Client::new();
        let url = self.url.join("login").unwrap();

        let response = client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<LoginResponse>()
            .await?;

        Ok(response)
    }

    pub async fn logout(&self, refresh_token: &Token) -> Result<LogoutResponse, Error> {
        let client = Client::new();
        let url = self.url.join("logout").unwrap();

        let response = client
            .post(url)
            .bearer_auth(refresh_token.to_string())
            .send()
            .await?
            .json::<LogoutResponse>()
            .await?;

        Ok(response)
    }

    pub async fn fetch_access_token(
        &self,
        refresh_token: &Token,
    ) -> Result<AccessTokenResponse, Error> {
        let client = Client::new();
        let request = AccessTokenRequest {
            grant_type: GrantType::RefreshToken,
            refresh_token: refresh_token.clone(),
        };
        let url = self.url.join("token").unwrap();

        let response = client
            .post(url)
            .form(&request)
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await?;

        Ok(response)
    }

    pub async fn whoami(&self, access_token: &Token) -> Result<WhoamiResponse, Error> {
        let client = Client::new();
        let url = self.url.join("whoami").unwrap();

        let response = client
            .get(url)
            .bearer_auth(access_token.to_string())
            .send()
            .await?
            .json::<WhoamiResponse>()
            .await?;

        Ok(response)
    }
}
