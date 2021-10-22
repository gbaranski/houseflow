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

    fn preprocess(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }

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

    fn parse(s: &str) -> Result<Self, Error> {
        use regex::Regex;

        let re = Regex::new(r"\$\{([a-zA-Z_]+)\}").unwrap();
        let s = re.replace_all(s, |caps: &regex::Captures| {
            let (pos, name) = {
                let name_match = caps.get(1).unwrap();
                let pos = name_match.start();
                let name = name_match.as_str();
                (pos, name)
            };
            match std::env::var(name) {
                Ok(env) => env,
                Err(std::env::VarError::NotPresent) => panic!(
                    "environment variable named {} from configuration file at {} is not defined",
                    name,
                    pos
                ),
                Err(std::env::VarError::NotUnicode(_)) => panic!(
                    "environment variable named {} from configuration file at {} is not valid unicode",
                    name,
                    pos
                ),
            }
        });
        let config: Self = toml::from_str(&s)?;
        config.validate().map_err(Error::Validation)?;

        Ok(config)
    }

    #[cfg(feature = "fs")]
    fn read(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }

    fn default_path() -> std::path::PathBuf {
        xdg::BaseDirectories::with_prefix("houseflow")
            .unwrap()
            .get_config_home()
            .join(Self::DEFAULT_FILE)
    }
}

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
    use tracing_subscriber::EnvFilter;

    let env_filter = match std::env::var(LOG_ENV) {
        Ok(env) => env,
        Err(std::env::VarError::NotPresent) => "info".to_string(),
        Err(std::env::VarError::NotUnicode(_)) => panic!(
            "{} environment variable is not valid unicode and can't be read",
            LOG_ENV
        ),
    };
    let env_filter = EnvFilter::from_str(&env_filter)
        .unwrap_or_else(|err| panic!("invalid `{}` environment variable {}", LOG_ENV, err));

    if hide_timestamp {
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .without_time()
            .init()
    } else {
        tracing_subscriber::fmt().with_env_filter(env_filter).init()
    };
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
