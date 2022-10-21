#[cfg(feature = "server-auth")]
pub mod auth;

#[cfg(feature = "server-meta")]
pub mod meta;

#[cfg(feature = "server-lighthouse")]
pub mod lighthouse;

use crate::Error;
use houseflow_config::client::Config;
use houseflow_types::token::Token;
use houseflow_types::token::TokenClaims;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use url::Url;

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: Default::default(),
        }
    }

    pub(crate) async fn post<B, E>(
        &self,
        url: Url,
        body: &impl Serialize,
    ) -> Result<Result<B, E>, Error>
    where
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let request = self.client.post(url).json(body);
        send_request(request).await
    }

    #[allow(dead_code)]
    pub(crate) async fn get<B, E>(
        &self,
        url: Url,
        body: &impl Serialize,
    ) -> Result<Result<B, E>, Error>
    where
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let request = self.client.get(url).json(body);
        send_request(request).await
    }

    pub(crate) async fn post_with_token<TC, B, E>(
        &self,
        url: Url,
        body: &impl Serialize,
        token: &Token<TC>,
    ) -> Result<Result<B, E>, Error>
    where
        TC: TokenClaims,
        B: DeserializeOwned,
        E: DeserializeOwned,
    {
        let request = self.client.post(url).json(body).bearer_auth(token);
        send_request(request).await
    }

    pub(crate) async fn get_with_token<TC, B, E>(
        &self,
        url: Url,
        body: &impl Serialize,
        token: &Token<TC>,
    ) -> Result<Result<B, E>, Error>
    where
        TC: TokenClaims,
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
