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

#[cfg(any(test, feature = "device", feature = "client", feature = "server",))]
pub(crate) mod resolve_socket_address {
    use serde::{
        de::{self, Visitor},
        Deserializer, Serializer,
    };
    use std::net::SocketAddr;

    use std::fmt;
    pub fn serialize<S>(address: &SocketAddr, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(address.to_string().as_str())
    }

    struct SocketAddrVisitor;

    impl<'de> Visitor<'de> for SocketAddrVisitor {
        type Value = SocketAddr;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("valid socket address")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            use std::net::ToSocketAddrs;
            match v
                .to_socket_addrs()
                .map_err(|err| de::Error::custom(err.to_string()))?
                .next()
            {
                Some(addr) => Ok(addr),
                None => Err(de::Error::custom(
                    "didn't found any SocketAddr for given address",
                )),
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<std::net::SocketAddr, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SocketAddrVisitor)
    }
}
