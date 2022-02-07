use super::Client;
use crate::Error;
use houseflow_types::auth;
use houseflow_types::errors::ServerError;
use houseflow_types::token::AccessToken;
use houseflow_types::token::RefreshToken;
use reqwest::Url;

impl Client {
    fn auth_url(&self, path: &str) -> Url {
        self.config
            .server
            .url
            .join(&format!("auth/{}", path))
            .unwrap()
    }

    pub async fn login(
        &self,
        request: &auth::login::Request,
    ) -> Result<Result<auth::login::Response, ServerError>, Error> {
        let url = self.auth_url("login");
        self.post(url, request).await
    }

    pub async fn refresh_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<Result<auth::token::Response, ServerError>, Error> {
        let url = self.auth_url("refresh");
        self.post_with_token(url, &auth::token::Request {}, refresh_token)
            .await
    }

    pub async fn whoami(
        &self,
        access_token: &AccessToken,
    ) -> Result<Result<auth::whoami::Response, ServerError>, Error> {
        let url = self.auth_url("whoami");
        self.get_with_token(url, &auth::whoami::Request {}, access_token)
            .await
    }
}
