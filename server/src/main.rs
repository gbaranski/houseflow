use houseflow_config::defaults;
use houseflow_config::dynamic;
use houseflow_config::server::Config;
use houseflow_config::server::Controllers;
use houseflow_config::server::Mailers;
use houseflow_config::server::Providers;
use houseflow_config::Config as _;
use houseflow_config::Error as ConfigError;
use houseflow_server::clerk::Clerk;
use houseflow_server::mailer;
use houseflow_server::providers;
use houseflow_server::Arg;
use houseflow_server::ArgControllers;
use houseflow_server::ArgMailers;
use houseflow_server::ArgProviders;
use houseflow_server::Server;
use houseflow_server::{clerk, controllers};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    houseflow_config::log::init_with_config(houseflow_config::log::Config {
        hide_timestamp: std::env::var_os("HOUSEFLOW_LOG_HIDE_TIMESTAMP").is_some(),
    });
    let config_path = std::env::var("HOUSEFLOW_SERVER_CONFIG")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| Config::default_path());

    tracing::debug!("Config path: {:?}", config_path);

    let config = match Config::read(&config_path) {
        Ok(config) => config,
        Err(ConfigError::IO(err)) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                tracing::error!("Config file could not be found at {:?}", config_path);
                return Ok(());
            }
            _ => panic!("Read config IO Error: {}", err),
        },
        Err(err) => panic!("Config error: {}", err),
    };
    tracing::debug!("Config: {:#?}", config);

    let clerk = clerk::sled::Clerk::new(defaults::clerk_path())?;
    let clerk = Arc::new(clerk) as Arc<dyn Clerk>;

    let mailers = {
        let Mailers { smtp, dummy } = config.mailers.to_owned();
        ArgMailers {
            dummy: dummy.map(|_| {
                let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
                tokio::spawn(async move {
                    while let Some((to, code)) = receiver.recv().await {
                        tracing::info!("verification code for {}: {}", to, code);
                    }
                });
                mailer::dummy::new(sender)
            }),
            smtp: smtp.map(|smtp| {
                let config = mailer::smtp::Config {
                    host: smtp.url.host_str().unwrap().to_string(),
                    port: smtp.url.port().unwrap_or(465),
                    username: smtp.url.username().to_string(),
                    password: urlencoding::decode(smtp.url.password().unwrap())
                        .unwrap()
                        .to_string(),
                    from: smtp.from.parse().unwrap(),
                };
                mailer::smtp::new(config)
            }),
        }
    };

    let providers = {
        let Providers { lighthouse } = config.providers.to_owned();
        ArgProviders {
            dummy: None,
            lighthouse: match lighthouse {
                Some(lighthouse) => Some(Box::new(|master_controller| {
                    providers::lighthouse::new(master_controller, lighthouse)
                })),
                None => None,
            },
        }
    };

    let controllers = {
        let Controllers { meta } = config.controllers.to_owned();
        ArgControllers {
            meta: match meta {
                Some(_meta) => Some(Box::new(|_master_controller| controllers::meta::new())),
                None => None,
            },
        }
    };

    let server = Server::new(Arg {
        config: dynamic::Config::new(config),
        clerk,
        mailers,
        providers,
        controllers,
    })
    .await;
    server.run().await;
    Ok(())
}
