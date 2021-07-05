use crate::{Error, HouseflowAPI, get_with_token, post, post_with_token, send_request};
use houseflow_types::{
    auth,
    token::{AccessToken, RefreshToken},
};

#[cfg(feature = "auth")]
impl HouseflowAPI {
    pub async fn register(
        &self,
        request: &auth::register::Request,
    ) -> Result<auth::register::Response, Error> {
        let url = self.auth_url.join("register").unwrap();
        post(url, request).await
    }

    pub async fn login(
        &self,
        request: &auth::login::Request,
    ) -> Result<auth::login::Response, Error> {
        let url = self.auth_url.join("login").unwrap();
        post(url, request).await
    }

    pub async fn logout(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<auth::logout::Response, Error> {
        let url = self.auth_url.join("logout").unwrap();
        post_with_token(url, &auth::logout::Request {}, refresh_token).await
    }

    pub async fn fetch_access_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<auth::token::Response, Error> {
        let request = auth::token::Request {
            grant_type: auth::token::GrantType::RefreshToken,
            refresh_token: refresh_token.to_string(),
        };
        let client = reqwest::Client::new();
        let url = self.auth_url.join("token").unwrap();
        let request = client.post(url).form(&request);
        send_request(request).await
    }

    pub async fn whoami(
        &self,
        access_token: &AccessToken,
    ) -> Result<auth::whoami::Response, Error> {
        let url = self.auth_url.join("whoami").unwrap();
        get_with_token(url, &auth::whoami::Request {}, access_token).await
    }
}
