use anyhow::Context;
use houseflow_api::HouseflowAPI;
use houseflow_config::client::Config;
use houseflow_types::{
    token::{AccessToken, RefreshToken},
    Device,
};
use serde::{Deserialize, Serialize};
use szafka::Szafka;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tokens {
    pub(crate) access: String,
    pub(crate) refresh: String,
}

#[derive(Debug, Clone)]
pub struct CommandContext {
    pub config_path: std::path::PathBuf,
    pub config: Option<Config>,
    pub houseflow_api: Option<HouseflowAPI>,
    pub tokens: Szafka<Tokens>,
    pub devices: Szafka<Vec<Device>>,
}

impl CommandContext {
    pub async fn new(config_path: std::path::PathBuf) -> anyhow::Result<Self> {
        let ctx = CommandContext {
            config_path,
            config: None,
            houseflow_api: None,
            tokens: Szafka::new(houseflow_config::defaults::data_home().join("tokens")),
            devices: Szafka::new(houseflow_config::defaults::data_home().join("devices")),
        };
        Ok::<_, anyhow::Error>(ctx)
    }

    pub async fn config(&mut self) -> anyhow::Result<&Config> {
        match self.config {
            Some(ref config) => Ok(config),
            None => {
                let config = Config::get(&self.config_path)
                    .await
                    .context("read configuration")?;
                tracing::trace!("config loaded: {:#?}", config);
                self.config = Some(config);
                Ok(self.config.as_ref().unwrap())
            }
        }
    }

    pub async fn houseflow_api(&mut self) -> anyhow::Result<&HouseflowAPI> {
        match self.houseflow_api {
            Some(ref api) => Ok(api),
            None => {
                let config = self.config().await?;
                let api = HouseflowAPI::new(&config);
                self.houseflow_api = Some(api);
                Ok(self.houseflow_api.as_ref().unwrap())
            }
        }
    }

    pub async fn access_token(&mut self) -> anyhow::Result<AccessToken> {
        let tokens = self.tokens.get().await.with_context(|| "get tokens")?;
        let refresh_token = RefreshToken::decode_unsafe(&tokens.refresh)
            .with_context(|| "you may need to log in again using `houseflow auth login`")?;

        let access_token = AccessToken::decode_unsafe(&tokens.access);
        match access_token {
            Ok(token) => {
                tracing::debug!("cached access token is valid");
                Ok(token)
            }
            Err(err) => {
                tracing::debug!("token verify returned error: {}", err);
                tracing::debug!("cached access token is expired, fetching new one");
                let raw_fetched_access_token = self
                    .houseflow_api()
                    .await?
                    .refresh_token(&refresh_token)
                    .await??
                    .access_token;
                let fetched_access_token = AccessToken::decode_unsafe(&raw_fetched_access_token)?;
                let tokens = Tokens {
                    refresh: tokens.refresh,
                    access: raw_fetched_access_token,
                };

                self.tokens.save(&tokens).await?;
                Ok(fetched_access_token)
            }
        }
    }

    pub async fn refresh_token(&mut self) -> anyhow::Result<RefreshToken> {
        let tokens = self.tokens.get().await.with_context(|| "get tokens")?;
        RefreshToken::decode_unsafe(&tokens.refresh)
            .with_context(|| "you may need to log in again using `houseflow auth login`")
    }
}
