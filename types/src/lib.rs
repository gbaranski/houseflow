pub mod accessory;
pub mod client;
pub mod hive;
pub mod hub;
pub mod permission;
pub mod room;
pub mod structure;
pub mod user;

pub mod code;
pub mod errors;

#[cfg(feature = "auth")]
pub mod auth;

// #[cfg(feature = "fulfillment")]
// pub mod fulfillment;

#[cfg(feature = "lighthouse")]
pub mod lighthouse;

#[cfg(feature = "token")]
pub mod token;

#[cfg(feature = "token")]
pub mod serde_token_expiration {
    use chrono::Duration;
    use serde::de;
    use serde::de::Visitor;
    use serde::ser;

    pub struct TokenExpirationVisitor;

    impl<'de> Visitor<'de> for TokenExpirationVisitor {
        type Value = Option<Duration>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("duration in seconds")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let v: i64 = v.try_into().map_err(|err| {
                serde::de::Error::custom(&format!("u64 to i64 cast fail: {}", err))
            })?;
            self.visit_i64(v)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(Duration::seconds(value)))
        }

        fn visit_some<D>(self, d: D) -> Result<Option<Duration>, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            d.deserialize_i64(Self)
        }

        fn visit_none<E>(self) -> Result<Option<Duration>, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Option<Duration>, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *duration {
            Some(duration) => serializer.serialize_some(&duration.num_seconds()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Option<Duration>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_option(TokenExpirationVisitor)
    }
}
