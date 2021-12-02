#[cfg(feature = "auth")]
pub mod auth;

#[cfg(feature = "fulfillment")]
pub mod fulfillment;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("error occured with sending request: `{0}`")]
    ReqwestError(#[from] reqwest::Error),

    #[error("invalid response body, code: `{status_code}`, error: `{error}`, body: `{body}`")]
    InvalidResponseBody {
        error: Box<dyn std::error::Error + Send + Sync>,
        status_code: reqwest::StatusCode,
        body: String,
    },
}

use houseflow_config::client::Config;
use houseflow_types::token::Token;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use url::Url;

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    config: Config,
    auth_url: Url,
    fulfillment_url: Url,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            auth_url: config.server.url.join("auth/").unwrap(),
            fulfillment_url: config.server.url.join("fulfillment/internal").unwrap(),
            config,
            client: Default::default(),
        }
    }

    pub(crate) async fn post<B, E>(&self, url: Url, body: &impl Serialize) -> Result<Result<B, E>, Error>
    where
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let request = self.client.post(url).json(body);
        send_request(request).await
    }

    #[allow(dead_code)]
    pub(crate) async fn get<B, E>(&self, url: Url, body: &impl Serialize) -> Result<Result<B, E>, Error>
    where
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let request = self.client.get(url).json(body);
        send_request(request).await
    }

    pub(crate) async fn post_with_token<TP, B, E>(
        &self,
        url: Url,
        body: &impl Serialize,
        token: &Token<TP>,
    ) -> Result<Result<B, E>, Error>
    where
        TP: Serialize + DeserializeOwned,
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let request = self.client.post(url).json(body).bearer_auth(token);
        send_request(request).await
    }

    pub(crate) async fn get_with_token<TP, B, E>(
        &self,
        url: Url,
        body: &impl Serialize,
        token: &Token<TP>,
    ) -> Result<Result<B, E>, Error>
    where
        TP: Serialize + DeserializeOwned,
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let request = self.client.get(url).json(body).bearer_auth(token);
        send_request(request).await
    }
}

pub(crate) async fn send_request<B: DeserializeOwned, E: DeserializeOwned>(
    request: reqwest::RequestBuilder,
) -> Result<Result<B, E>, Error> {
    let response = request.send().await?;
    let status_code = response.status();
    let bytes = response.bytes().await?;
    let result = if status_code.is_success() {
        let parsed = serde_json::from_slice(&bytes).map_err(|err| Error::InvalidResponseBody {
            error: Box::new(err),
            status_code,
            body: String::from_utf8(bytes.to_vec()).unwrap(),
        })?;
        Ok(parsed)
    } else {
        let parsed = serde_json::from_slice(&bytes).map_err(|err| Error::InvalidResponseBody {
            error: Box::new(err),
            status_code,
            body: String::from_utf8(bytes.to_vec()).unwrap(),
        })?;
        Err(parsed)
    };
    Ok(result)
}
