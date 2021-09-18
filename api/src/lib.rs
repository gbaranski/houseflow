#[cfg(feature = "auth")]
mod auth;

#[cfg(feature = "auth")]
mod fulfillment;

#[cfg(feature = "auth")]
pub use crate::fulfillment::FulfillmentError;

#[cfg(any(feature = "auth", feature = "fulfillment"))]
use url::Url;

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

#[derive(Debug, Clone)]
pub struct HouseflowAPI {
    #[cfg(feature = "auth")]
    auth_url: Url,

    #[cfg(feature = "fulfillment")]
    fulfillment_url: Url,
}

use houseflow_config::client::Config;

impl HouseflowAPI {
    pub fn new(config: &Config) -> Self {
        Self {
            #[cfg(feature = "auth")]
            auth_url: config.server.url.join("auth/").unwrap(),

            #[cfg(feature = "fulfillment")]
            fulfillment_url: config.server.url.join("fulfillment/internal/").unwrap(),
        }
    }
}

#[cfg(any(feature = "auth", feature = "fulfillment"))]
mod utils {
    use super::Error;
    use houseflow_types::token::Token;
    use reqwest::Client;
    use serde::de::DeserializeOwned;
    use serde::ser::Serialize;
    use url::Url;

    pub(crate) async fn send_request<B: DeserializeOwned, E: DeserializeOwned>(
        request: reqwest::RequestBuilder,
    ) -> Result<Result<B, E>, Error> {
        let response = request.send().await?;
        let status_code = response.status();
        let result = if response.status().is_success() {
            let bytes = response.bytes().await?;
            let parsed =
                serde_json::from_slice(&bytes).map_err(|err| Error::InvalidResponseBody {
                    error: Box::new(err),
                    status_code,
                    body: String::from_utf8(bytes.to_vec()).unwrap(),
                })?;
            Ok(parsed)
        } else {
            let bytes = response.bytes().await?;
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

    pub(crate) async fn post<B, E>(url: Url, body: &impl Serialize) -> Result<Result<B, E>, Error>
    where
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let client = Client::new();
        let request = client.post(url).json(body);
        send_request(request).await
    }

    #[allow(dead_code)]
    pub(crate) async fn get<B, E>(url: Url, body: &impl Serialize) -> Result<Result<B, E>, Error>
    where
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let client = Client::new();
        let request = client.get(url).json(body);
        send_request(request).await
    }

    pub(crate) async fn post_with_token<TP, B, E>(
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
        send_request(request).await
    }

    pub(crate) async fn get_with_token<TP, B, E>(
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
        send_request(request).await
    }
}

#[cfg(any(feature = "auth", feature = "fulfillment"))]
pub(crate) use utils::*;
