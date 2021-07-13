pub mod execute;
pub mod query;
pub mod sync;

use serde::{Deserialize, Serialize};

use strum::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter, strum::Display)]
#[repr(u8)]
#[serde(rename_all = "UPPERCASE")]
pub enum DeviceStatus {
    /// Confirm that the command succeeded.
    Success,

    /// Target device is in offline state or unreachable.
    Offline,

    /// There is an issue or alert associated with a query.
    /// The query could succeed or fail.
    /// This status type is typically set when you want to send additional information about another connected device.
    Exceptions,

    /// Target device is unable to perform the command.
    Error,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentRequest {
    pub request_id: String,
    pub inputs: Vec<IntentRequestInput>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "intent", content = "payload")]
pub enum IntentRequestInput {
    #[serde(rename = "action.devices.SYNC")]
    Sync,

    #[serde(rename = "action.devices.QUERY")]
    Query(query::request::Payload),

    #[serde(rename = "action.devices.EXECUTE")]
    Execute(execute::request::Payload),

    #[serde(rename = "action.devices.DISCONNECT")]
    Disconnect,
}

pub type IntentResponse = Result<IntentResponseBody, IntentResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IntentResponseBody {
    Sync {
        request_id: String,
        payload: sync::response::Payload,
    },
    Query {
        request_id: String,
        payload: query::response::Payload,
    },
    Execute {
        request_id: String,
        payload: execute::response::Payload,
    },
    Disconnect,
}

use crate::{lighthouse, token};

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum IntentResponseError {
    #[error("internal error: {0}")]
    InternalError(#[from] crate::InternalServerError),

    #[error("token error: {0}")]
    TokenError(#[from] token::Error),

    #[error("no device permission")]
    NoDevicePermission,

    #[error("error with device communication: {0}")]
    DeviceCommunicationError(#[from] lighthouse::DeviceCommunicationError),
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for IntentResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TokenError(_) => StatusCode::UNAUTHORIZED,
            Self::NoDevicePermission => StatusCode::UNAUTHORIZED,
            Self::DeviceCommunicationError(_) => StatusCode::BAD_GATEWAY,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        crate::json_error_response(self.status_code(), self)
    }
}

mod serde_device_type {
    const PREFIX: &str = "action.devices.types";

    pub fn serialize<S: serde::Serializer, T: std::fmt::Display>(
        val: T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}.{}", PREFIX, val.to_string().to_uppercase()))
    }

    pub fn deserialize<
        'de,
        D: serde::Deserializer<'de>,
        TE: std::fmt::Display,
        T: std::str::FromStr<Err = TE>,
    >(
        deserializer: D,
    ) -> Result<T, D::Error> {
        struct TVisitor<TE: std::fmt::Display, T: std::str::FromStr<Err = TE>> {
            phantom: std::marker::PhantomData<T>,
        }

        impl<'de, TE: std::fmt::Display, T: std::str::FromStr<Err = TE>> serde::de::Visitor<'de>
            for TVisitor<TE, T>
        {
            type Value = T;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_fmt(format_args!("value prefixed with {}", PREFIX))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let without_prefix = v.replace(&format!("{}.", PREFIX), "");
                T::from_str(without_prefix.as_str()).map_err(|err| E::custom(err))
            }
        }

        deserializer.deserialize_str(TVisitor {
            phantom: std::marker::PhantomData::default(),
        })
    }
}

mod serde_device_traits {
    const PREFIX: &str = "action.devices.traits";

    pub fn serialize<S: serde::Serializer, T: std::fmt::Display>(
        val: &Vec<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(val.len()))?;
        for val in val {
            seq.serialize_element(&format!("{}.{}", PREFIX, val.to_string()))?;
        }
        seq.end()
    }

    pub fn deserialize<
        'de,
        D: serde::Deserializer<'de>,
        TE: std::fmt::Display,
        T: std::str::FromStr<Err = TE>,
    >(
        deserializer: D,
    ) -> Result<Vec<T>, D::Error> {
        struct TVisitor<TE: std::fmt::Display, T: std::str::FromStr<Err = TE>> {
            phantom: std::marker::PhantomData<T>,
        }

        impl<'de, TE: std::fmt::Display, T: std::str::FromStr<Err = TE>> serde::de::Visitor<'de>
            for TVisitor<TE, T>
        {
            type Value = Vec<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_fmt(format_args!("device traits prefixed with {}", PREFIX))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut vec: Vec<T> = if let Some(size) = seq.size_hint() {
                    Vec::with_capacity(size)
                } else {
                    Vec::new()
                };

                while let Some(e) = seq.next_element::<&str>()? {
                    let without_prefix = e.replace(&format!("{}.", PREFIX), "");
                    let t = T::from_str(&without_prefix)
                        .map_err(|err| serde::de::Error::custom(err))?;
                    vec.push(t);
                }
                Ok(vec)
            }
        }

        deserializer.deserialize_seq(TVisitor {
            phantom: std::marker::PhantomData::default(),
        })
    }
}
