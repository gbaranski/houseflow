use crate::Error;
use houseflow_config::client::Config;
use houseflow_types::auth;
use houseflow_types::errors::ServerError;
use houseflow_types::fulfillment;
use houseflow_types::token::AccessToken;
use houseflow_types::token::RefreshToken;
use url::Url;

#[derive(Debug, Clone)]
pub struct Client {
    auth_url: Url,
    fulfillment_url: Url,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            auth_url: config.server.url.join("auth/").unwrap(),
            fulfillment_url: config.server.url.join("fulfillment/internal").unwrap(),
        }
    }
}

#[cfg(feature = "auth")]
impl Client {
    pub fn login(
        &self,
        request: &auth::login::Request,
    ) -> Result<Result<auth::login::Response, ServerError>, Error> {
        let url = self.auth_url.join("login").unwrap();
        post(url, request)
    }

    pub fn refresh_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<Result<auth::token::Response, ServerError>, Error> {
        let url = self.auth_url.join("refresh").unwrap();
        post_with_token(url, &auth::token::Request {}, refresh_token)
    }

    pub fn whoami(
        &self,
        access_token: &AccessToken,
    ) -> Result<Result<auth::whoami::Response, ServerError>, Error> {
        let url = self.auth_url.join("whoami").unwrap();
        get_with_token(url, &auth::whoami::Request {}, access_token)
    }
}

#[cfg(feature = "fulfillment")]
impl Client {
    pub fn sync(
        &self,
        access_token: &AccessToken,
    ) -> Result<Result<fulfillment::sync::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("sync").unwrap();
        get_with_token(url, &fulfillment::sync::Request {}, access_token)
    }

    pub fn execute(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::execute::Request,
    ) -> Result<Result<fulfillment::execute::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("execute").unwrap();
        post_with_token(url, request, access_token)
    }

    pub fn query(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::query::Request,
    ) -> Result<Result<fulfillment::query::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("query").unwrap();
        post_with_token(url, request, access_token)
    }
}

#[cfg(any(feature = "auth", feature = "fulfillment"))]
pub(crate) use utils::*;

#[cfg(any(feature = "auth", feature = "fulfillment"))]
mod utils {
    use crate::Error;
    use houseflow_types::token::Token;
    use reqwest::blocking::Client;
    use serde::de::DeserializeOwned;
    use serde::ser::Serialize;
    use url::Url;

    pub(crate) fn send_request<B: DeserializeOwned, E: DeserializeOwned>(
        request: reqwest::blocking::RequestBuilder,
    ) -> Result<Result<B, E>, Error> {
        let response = request.send()?;
        let status_code = response.status();
        let bytes = response.bytes()?;
        let result = if status_code.is_success() {
            let parsed =
                serde_json::from_slice(&bytes).map_err(|err| Error::InvalidResponseBody {
                    error: Box::new(err),
                    status_code,
                    body: String::from_utf8(bytes.to_vec()).unwrap(),
                })?;
            Ok(parsed)
        } else {
            let parsed =
                serde_json::from_slice(&bytes).map_err(|err| Error::InvalidResponseBody {
                    error: Box::new(err),
                    status_code,
                    body: String::from_utf8(bytes.to_vec()).unwrap(),
                })?;
            Err(parsed)
        };
        Ok(result)
    }

    pub(crate) fn post<B, E>(url: Url, body: &impl Serialize) -> Result<Result<B, E>, Error>
    where
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let client = Client::new();
        let request = client.post(url).json(body);
        send_request(request)
    }

    #[allow(dead_code)]
    pub(crate) fn get<B, E>(url: Url, body: &impl Serialize) -> Result<Result<B, E>, Error>
    where
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let client = Client::new();
        let request = client.get(url).json(body);
        send_request(request)
    }

    pub(crate) fn post_with_token<TP, B, E>(
        url: Url,
        body: &impl Serialize,
        token: &Token<TP>,
    ) -> Result<Result<B, E>, Error>
    where
        TP: Serialize + DeserializeOwned,
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let client = Client::new();
        let request = client.post(url).json(body).bearer_auth(token);
        send_request(request)
    }

    pub(crate) fn get_with_token<TP, B, E>(
        url: Url,
        body: &impl Serialize,
        token: &Token<TP>,
    ) -> Result<Result<B, E>, Error>
    where
        TP: Serialize + DeserializeOwned,
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let client = Client::new();
        let request = client.get(url).json(body).bearer_auth(token);
        send_request(request)
    }
}
