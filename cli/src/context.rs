use anyhow::Context;
use houseflow_api::ServerClient;
use houseflow_config::client::Config;
use houseflow_config::Config as _;
use houseflow_types::accessory::Accessory;
use houseflow_types::token::AccessToken;
use houseflow_types::token::RefreshToken;
use serde::Deserialize;
use serde::Serialize;
use szafka::Szafka;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tokens {
    pub(crate) access: String,
    pub(crate) refresh: String,
}

#[derive(Debug, Clone)]
pub struct CommandContext {
    config_path: std::path::PathBuf,
    config: Option<Config>,
    server_client: Option<ServerClient>,
    pub tokens: Szafka<Tokens>,
    pub devices: Szafka<Vec<Accessory>>,
}

impl CommandContext {
    pub fn new(config_path: std::path::PathBuf) -> anyhow::Result<Self> {
        let ctx = CommandContext {
            config_path,
            config: None,
            server_client: None,
            tokens: Szafka::new(houseflow_config::defaults::data_home().join("tokens")),
            devices: Szafka::new(houseflow_config::defaults::data_home().join("devices")),
        };
        Ok::<_, anyhow::Error>(ctx)
    }

    pub fn config(&mut self) -> anyhow::Result<&Config> {
        match self.config {
            Some(ref config) => Ok(config),
            None => {
                let config = if self.config_path.exists() {
                    Config::read(&self.config_path).context("read configuration")?
                } else {
                    Config::default()
                };
                tracing::trace!("config loaded: {:#?}", config);
                self.config = Some(config);
                Ok(self.config.as_ref().unwrap())
            }
        }
    }

    pub fn server_client(&mut self) -> anyhow::Result<&ServerClient> {
        match self.server_client {
            Some(ref api) => Ok(api),
            None => {
                let config = self.config()?;
                let client = ServerClient::new(config.clone());
                self.server_client = Some(client);
                Ok(self.server_client.as_ref().unwrap())
            }
        }
    }

    pub async fn access_token(&mut self) -> anyhow::Result<AccessToken> {
        let tokens = match self.tokens.get() {
            Ok(tokens) => tokens,
            Err(szafka::Error::OpenFileError(err)) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(anyhow::anyhow!(
                        "Tokens not found on disk. You need to log in."
                    ))
                }
                _ => return Err(err.into()),
            },
            Err(err) => return Err(err).context("Get tokens error"),
        };
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
                let raw_fetched_access_token =
                    self.server_client()?.refresh_token(&refresh_token).await??.access_token;
                let fetched_access_token = AccessToken::decode_unsafe(&raw_fetched_access_token)?;
                let tokens = Tokens {
                    refresh: tokens.refresh,
                    access: raw_fetched_access_token,
                };

                self.tokens.save(&tokens)?;
                Ok(fetched_access_token)
            }
        }
    }

    pub fn refresh_token(&mut self) -> anyhow::Result<RefreshToken> {
        let tokens = self.tokens.get().with_context(|| "get tokens")?;
        RefreshToken::decode_unsafe(&tokens.refresh)
            .with_context(|| "you may need to log in again using `houseflow auth login`")
    }
}
