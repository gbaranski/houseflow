use super::{Error, HouseflowAPI};
use houseflow_types::auth;
use houseflow_types::token::{RefreshToken, AccessToken};
use reqwest::Client;

#[cfg(feature = "auth")]
impl HouseflowAPI {
    pub async fn register(&self, request: auth::register::Request) -> Result<auth::register::Response, Error> {
        let client = Client::new();
        let url = self.auth_url.join("register").unwrap();

        let response = client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<auth::register::Response>()
            .await?;

        Ok(response)
    }

    pub async fn login(&self, request: auth::login::Request) -> Result<auth::login::Response, Error> {
        let client = Client::new();
        let url = self.auth_url.join("login").unwrap();

        let response = client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<auth::login::Response>()
            .await?;

        Ok(response)
    }

    pub async fn logout(&self, refresh_token: &RefreshToken) -> Result<auth::logout::Response, Error> {
        let client = Client::new();
        let url = self.auth_url.join("logout").unwrap();

        let response = client
            .post(url)
            .bearer_auth(refresh_token.to_string())
            .send()
            .await?
            .json::<auth::logout::Response>()
            .await?;

        Ok(response)
    }

    pub async fn fetch_access_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<auth::token::Response, Error> {
        let client = Client::new();
        let request = auth::token::Request {
            grant_type: auth::token::GrantType::RefreshToken,
            refresh_token: refresh_token.to_string(),
        };
        let url = self.auth_url.join("token").unwrap();

        let response = client
            .post(url)
            .form(&request)
            .send()
            .await?
            .json::<auth::token::Response>()
            .await?;

        Ok(response)
    }

    pub async fn whoami(&self, access_token: &AccessToken) -> Result<auth::whoami::Response, Error> {
        let client = Client::new();
        let url = self.auth_url.join("whoami").unwrap();

        let response = client
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<auth::whoami::Response>()
            .await?;

        Ok(response)
    }
}
