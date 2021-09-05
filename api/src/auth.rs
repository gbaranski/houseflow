use crate::get_with_token;
use crate::post;
use crate::post_with_token;
use crate::Error;
use crate::HouseflowAPI;
use houseflow_types::auth;
use houseflow_types::errors::ServerError;
use houseflow_types::token::AccessToken;
use houseflow_types::token::RefreshToken;

#[cfg(feature = "auth")]
impl HouseflowAPI {
    pub async fn login(
        &self,
        request: &auth::login::Request,
    ) -> Result<Result<auth::login::Response, ServerError>, Error> {
        let url = self.auth_url.join("login").unwrap();
        post(url, request).await
    }

    pub async fn refresh_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<Result<auth::token::Response, ServerError>, Error> {
        let url = self.auth_url.join("refresh").unwrap();
        post_with_token(url, &auth::token::Request {}, refresh_token).await
    }

    pub async fn whoami(
        &self,
        access_token: &AccessToken,
    ) -> Result<Result<auth::whoami::Response, ServerError>, Error> {
        let url = self.auth_url.join("whoami").unwrap();
        get_with_token(url, &auth::whoami::Request {}, access_token).await
    }
}
