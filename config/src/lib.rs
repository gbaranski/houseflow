pub mod defaults;

#[cfg(any(test, feature = "server"))]
pub mod server;

#[cfg(any(test, feature = "device"))]
pub mod device;

#[cfg(any(test, feature = "client"))]
pub mod client;

pub trait Config: serde::de::DeserializeOwned + serde::ser::Serialize {
    const DEFAULT_TOML: &'static str;
    const DEFAULT_FILE: &'static str;

    #[cfg(feature = "fs")]
    fn write_defaults(path: impl AsRef<std::path::Path>) -> Result<(), Error> {
        use std::io::Write;

        let path = path.as_ref();
        if path.parent().is_none() || !path.parent().unwrap().exists() {
            let mut comps = path.components();
            comps.next_back();
            std::fs::create_dir_all(comps.as_path())?;
        }
        let mut file = std::fs::File::create(path)?;
        file.write_all(Self::DEFAULT_TOML.as_bytes())?;
        Ok(())
    }

    #[cfg(feature = "fs")]
    fn read(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        config.validate().map_err(Error::Validation)?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }

    fn default_path() -> std::path::PathBuf {
        xdg::BaseDirectories::with_prefix("houseflow")
            .unwrap()
            .get_config_home()
            .join(Self::DEFAULT_FILE)
    }
}

#[cfg(feature = "fs")]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io: {0}")]
    IO(#[from] std::io::Error),

    #[error("toml deserialize: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("toml serialize: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("validation: {0}")]
    Validation(String),

}

pub fn init_logging(hide_timestamp: bool) {
    const LOG_ENV: &str = "HOUSEFLOW_LOG";
    use std::str::FromStr;
    use tracing::Level;

    let level = std::env::var(LOG_ENV)
        .map(|env| {
            Level::from_str(env.to_uppercase().as_str())
                .unwrap_or_else(|err| panic!("invalid `{}` environment variable {}", LOG_ENV, err))
        })
        .unwrap_or(Level::INFO);

    if hide_timestamp {
        tracing_subscriber::fmt()
            .with_max_level(level)
            .without_time()
            .init();
    } else {
        tracing_subscriber::fmt().with_max_level(level).init();
    }
}

#[allow(dead_code)]
pub(crate) mod serde_hostname {
    use serde::de;
    use serde::de::Visitor;
    use serde::Deserializer;
    use serde::Serializer;
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
