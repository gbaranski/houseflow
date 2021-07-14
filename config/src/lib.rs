pub mod defaults;

#[cfg(any(test, feature = "server"))]
pub mod server;

#[cfg(any(test, feature = "device"))]
pub mod device;

#[cfg(any(test, feature = "client"))]
pub mod client;

pub struct Config {
    #[cfg(feature = "server")]
    pub server: server::Config,

    #[cfg(feature = "client")]
    pub client: client::Config,

    #[cfg(feature = "device")]
    pub device: device::Config,
}

#[cfg(feature = "fs")]
pub async fn read_file<T: serde::de::DeserializeOwned>(
    path: std::path::PathBuf,
) -> Result<T, std::io::Error> {
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "config file not found at {}",
                path.to_str().unwrap_or("INVALID_PATH")
            ),
        ));
    }

    let content = tokio::fs::read_to_string(path).await?;
    let content = content.as_str();
    let config: T = toml::from_str(content)?;

    Ok(config)
}


pub fn init_logging() {
    const LOG_ENV: &str = "HOUSEFLOW_LOG";
    use tracing::Level;
    use std::str::FromStr;


    let level = std::env::var(LOG_ENV)
        .map(|env| {
            Level::from_str(env.to_uppercase().as_str())
                .expect(&format!("invalid `{}` environment variable", LOG_ENV))
        })
        .unwrap_or(Level::INFO);

    tracing_subscriber::fmt().with_max_level(level).init();
}

#[allow(dead_code)]
pub(crate) mod serde_hostname {
    use serde::{
        de::{self, Visitor},
        Deserializer, Serializer,
    };
    use url::Host;

    use std::fmt;
    pub fn serialize<S>(host: &Host, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(host.to_string().as_str())
    }

    struct HostnameVisitor;

    impl<'de> Visitor<'de> for HostnameVisitor {
        type Value = Host;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("valid hostname")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Host::parse(v).map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Host, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HostnameVisitor)
    }
}
