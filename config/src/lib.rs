pub mod defaults;

#[cfg(any(test, feature = "server"))]
pub mod server;

#[cfg(any(test, feature = "device"))]
pub mod device;

#[cfg(any(test, feature = "client"))]
pub mod client;

pub trait Config: serde::de::DeserializeOwned + serde::ser::Serialize {
    #[cfg(feature = "fs")]
    fn read(path: impl AsRef<std::path::Path>) -> Result<Self, ReadFileError> {
        read_file(path, Self::default_yaml)
    }

    fn default_path() -> std::path::PathBuf;

    fn default_yaml() -> String;
}

#[cfg(feature = "fs")]
#[derive(Debug, thiserror::Error)]
pub enum ReadFileError {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("yaml error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

#[cfg(feature = "fs")]
pub fn read_file<T: serde::de::DeserializeOwned>(
    path: impl AsRef<std::path::Path>,
    defaults: impl FnOnce() -> String,
) -> Result<T, ReadFileError> {
    use std::io::Write;

    let path = path.as_ref();
    let content = if path.exists() {
        std::fs::read_to_string(path)?
    } else {
        let defaults = defaults();
        if path.parent().is_none() || !path.parent().unwrap().exists() {
            let mut comps = path.components();
            comps.next_back();
            std::fs::create_dir_all(comps.as_path())?;
        }
        let mut file = std::fs::File::create(path)?;
        file.write_all(defaults.as_bytes())?;
        defaults
    };

    let config: T = serde_yaml::from_str(&content)?;

    Ok(config)
}

pub fn init_logging() {
    const LOG_ENV: &str = "HOUSEFLOW_LOG";
    use std::str::FromStr;
    use tracing::Level;

    let level = std::env::var(LOG_ENV)
        .map(|env| {
            Level::from_str(env.to_uppercase().as_str())
                .unwrap_or_else(|err| panic!("invalid `{}` environment variable {}", LOG_ENV, err))
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
