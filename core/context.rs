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
    pub config: Config,
    pub houseflow_api: HouseflowAPI,
    pub tokens: Szafka<Tokens>,
    pub devices: Szafka<Vec<Device>>,
}

impl CommandContext {
    pub async fn new(config_path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        use anyhow::Context;

        let config = Config::get(config_path)
            .await
            .context("read configuration")?;

        tracing::trace!("config loaded: {:#?}", config);
        let ctx = CommandContext {
            config: config.clone(),
            houseflow_api: HouseflowAPI::new(&config),
            tokens: Szafka::new(houseflow_config::defaults::data_home().join("tokens")),
            devices: Szafka::new(houseflow_config::defaults::data_home().join("devices")),
        };
        Ok::<_, anyhow::Error>(ctx)
    }

    pub async fn access_token(&self) -> anyhow::Result<AccessToken> {
        use anyhow::Context;

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
                    .houseflow_api
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

    pub async fn refresh_token(&self) -> anyhow::Result<RefreshToken> {
        use anyhow::Context;

        let tokens = self.tokens.get().await.with_context(|| "get tokens")?;
        RefreshToken::decode_unsafe(&tokens.refresh)
            .with_context(|| "you may need to log in again using `houseflow auth login`")
    }
}
