use crate::{get_with_token, post, post_with_token, Error, HouseflowAPI};
use houseflow_types::{
    auth,
    token::{AccessToken, RefreshToken},
    ServerError,
};

#[cfg(feature = "auth")]
impl HouseflowAPI {
    pub async fn register(
        &self,
        request: &auth::register::Request,
    ) -> Result<Result<auth::register::Response, ServerError>, Error> {
        let url = self.auth_url.join("register").unwrap();
        post(url, request).await
    }

    pub async fn login(
        &self,
        request: &auth::login::Request,
    ) -> Result<Result<auth::login::Response, ServerError>, Error> {
        let url = self.auth_url.join("login").unwrap();
        post(url, request).await
    }

    pub async fn logout(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<Result<auth::logout::Response, ServerError>, Error> {
        let url = self.auth_url.join("logout").unwrap();
        post_with_token(url, &auth::logout::Request {}, refresh_token).await
    }

    pub async fn refresh_token(
        &self,
        refresh_token: &RefreshToken,
        let request = auth::token::Request {
            refresh_token: refresh_token.to_string(),
        };
        let client = reqwest::Client::new();
    ) -> Result<Result<auth::token::Response, ServerError>, Error> {
        let url = self.auth_url.join("refresh_token").unwrap();
        let request = client.post(url).json(&request);
        send_request(request).await
    }

    pub async fn whoami(
        &self,
        access_token: &AccessToken,
    ) -> Result<Result<auth::whoami::Response, ServerError>, Error> {
        let url = self.auth_url.join("whoami").unwrap();
        get_with_token(url, &auth::whoami::Request {}, access_token).await
    }
}
