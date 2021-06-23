mod defaults;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "device")]
pub mod device;

#[cfg(feature = "client")]
pub mod client;

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
                Some(addr) => Ok(Self::Value::from(addr)),
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
