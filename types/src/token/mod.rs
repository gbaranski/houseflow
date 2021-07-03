use crate::{Credential, UserID};
use chrono::{DateTime, Utc};
use serde::{de, ser, Deserialize, Serialize};

pub type RefreshTokenID = Credential<16>;

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("decode error: {0}")]
    Decode(#[from] DecodeError),

    #[error("encode error: {0}")]
    Encode(#[from] EncodeError),

    #[error("decode header error: {0}")]
    DecodeHeader(#[from] DecodeHeaderError),

    #[error("validation error: {0}")]
    Validation(#[from] ValidationError),
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum DecodeError {
    #[error("missing header")]
    MissingHeader,

    #[error("missing payload")]
    MissingPayload,

    #[error("missing signature")]
    MissingSignature,

    #[error("invalid json: {0}")]
    InvalidJSON(String),

    #[error("invalid encoding: `{0}`")]
    InvalidEncoding(String),

    #[error("invalid signature")]
    InvalidSignature,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum EncodeError {
    #[error("json error: {0}")]
    JsonError(String),
}

impl From<serde_json::Error> for EncodeError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err.to_string())
    }
}

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecodeHeaderError {
    #[error("header is missing")]
    MissingHeader,

    #[error("invalid header encoding: {0}")]
    InvalidEncoding(String),

    #[error("invalid header value syntax")]
    InvalidSyntax,

    #[error("invalid header schema: {0}")]
    InvalidSchema(String),
}

#[cfg(feature = "actix")]
impl Error {
    pub fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;
        match self {
            Error::Decode(_) => StatusCode::BAD_REQUEST,
            Error::Encode(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DecodeHeader(_) => StatusCode::BAD_REQUEST,
            Error::Validation(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum ValidationError {
    #[error("token is expired since {seconds} seconds")]
    Expired { seconds: u64 },
}

pub type Key = Vec<u8>;
pub type Signature = Vec<u8>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Algorithm {
    /// HMAC using SHA-256
    HS256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    alg: Algorithm,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessToken {
    pub sub: UserID,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub exp: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefreshToken {
    pub tid: RefreshTokenID,
    pub sub: UserID,

    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub exp: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BasePayload {
    #[serde(with = "chrono::serde::ts_seconds_option")]
    exp: Option<DateTime<Utc>>,
}

fn base64_encode(val: &[u8]) -> String {
    base64::encode_config(&val, base64::URL_SAFE_NO_PAD)
}

fn base64_decode(val: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode_config(val, base64::URL_SAFE_NO_PAD)
}

fn encode_part<T: ser::Serialize>(val: &T) -> Result<String, serde_json::Error> {
    let string = serde_json::to_string(val)?;
    Ok(base64_encode(string.as_bytes()))
}

fn decode_part<T: de::DeserializeOwned>(val: &str) -> Result<T, DecodeError> {
    let json_bytes =
        base64_decode(val).map_err(|err| DecodeError::InvalidEncoding(err.to_string()))?;
    let json = String::from_utf8(json_bytes)
        .map_err(|err| DecodeError::InvalidEncoding(err.to_string()))?;
    serde_json::from_str(&json).map_err(|err| DecodeError::InvalidJSON(err.to_string()))
}

use ring::hmac;

pub fn encode<P: ser::Serialize>(key: &Key, payload: &P) -> Result<String, EncodeError> {
    const ALGORITHM: Algorithm = Algorithm::HS256; // that can be changed in the future
    const HEADER: Header = Header { alg: ALGORITHM };

    let raw_header = encode_part(&HEADER)?;
    let raw_payload = encode_part(&payload)?;
    let message = [raw_header, raw_payload].join(".");
    let raw_signature = match ALGORITHM {
        Algorithm::HS256 => {
            let key = hmac::Key::new(hmac::HMAC_SHA256, key);
            base64_encode(hmac::sign(&key, message.as_bytes()).as_ref())
        }
    };

    Ok([message, raw_signature].join("."))
}

fn validate(base_payload: &BasePayload) -> Result<(), ValidationError> {
    if let Some(exp) = base_payload.exp {
        let difference = Utc::now().timestamp() - exp.timestamp();
        if difference > 0 {
            return Err(ValidationError::Expired {
                seconds: difference as u64,
            });
        }
    }

    Ok(())
}

pub fn decode<P: de::DeserializeOwned>(key: &Key, token: &str) -> Result<P, Error> {
    let mut iter = token.split(".");
    let raw_header = iter.next().ok_or(DecodeError::MissingHeader)?;
    let raw_payload = iter.next().ok_or(DecodeError::MissingPayload)?;
    let raw_signature = iter.next().ok_or(DecodeError::MissingSignature)?;

    let header = decode_part::<Header>(raw_header)?;
    let signature = base64_decode(raw_signature)
        .map_err(|err| DecodeError::InvalidEncoding(err.to_string()))?;

    let is_signature_valid = match header.alg {
        Algorithm::HS256 => {
            let key = hmac::Key::new(hmac::HMAC_SHA256, key);
            hmac::verify(
                &key,
                [raw_header, raw_payload].join(".").as_bytes(),
                &signature,
            )
            .is_ok()
        }
    };
    if !is_signature_valid {
        return Err(Error::Decode(DecodeError::InvalidSignature));
    }

    let payload_base = decode_part::<BasePayload>(raw_payload)?;
    validate(&payload_base)?;
    let payload = decode_part::<P>(raw_payload)?;

    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::SubsecRound;
    use rand::random;

    fn get_key() -> Key {
        use rand::RngCore;
        let mut bytes = [0; 32];
        rand::thread_rng().fill_bytes(&mut bytes);

        Vec::from(bytes)
    }

    mod at {
        use super::*;

        #[test]
        fn valid() {
            let key = get_key();
            let token = AccessToken {
                sub: random(),
                exp: Utc::now().round_subsecs(0) + chrono::Duration::hours(1),
            };
            let encoded = encode(&key, &token).unwrap();
            let decoded = decode::<AccessToken>(&key, &encoded).unwrap();
            assert_eq!(token, decoded);
        }

        #[test]
        fn expired() {
            let key = get_key();
            let expired_by = chrono::Duration::hours(1);
            let token = AccessToken {
                sub: random(),
                exp: Utc::now() - expired_by,
            };
            let encoded = encode(&key, &token).unwrap();
            let err = decode::<AccessToken>(&key, &encoded).unwrap_err();
            assert_eq!(
                err,
                Error::Validation(ValidationError::Expired {
                    seconds: expired_by.num_seconds() as u64
                })
            );
        }

        #[test]
        fn invalid_signature() {
            let valid_key = get_key();
            let invalid_key = get_key();
            let expired_by = chrono::Duration::hours(1);
            let token = AccessToken {
                sub: random(),
                exp: Utc::now() - expired_by,
            };
            let encoded = encode(&valid_key, &token).unwrap();
            let err = decode::<AccessToken>(&invalid_key, &encoded).unwrap_err();
            assert_eq!(err, Error::Decode(DecodeError::InvalidSignature));
        }
    }

    mod rt {
        use super::*;

        #[test]
        fn valid_with_exp() {
            let key = get_key();
            let token = RefreshToken {
                sub: random(),
                exp: Some(Utc::now().round_subsecs(0) + chrono::Duration::hours(1)),
                tid: random(),
            };
            let encoded = encode(&key, &token).unwrap();
            let decoded = decode::<RefreshToken>(&key, &encoded).unwrap();
            assert_eq!(token, decoded);
        }

        #[test]
        fn valid_without_exp() {
            let key = get_key();
            let token = RefreshToken {
                sub: random(),
                exp: None,
                tid: random(),
            };
            let encoded = encode(&key, &token).unwrap();
            let decoded = decode::<RefreshToken>(&key, &encoded).unwrap();
            assert_eq!(token, decoded);
        }

        #[test]
        fn expired() {
            let key = get_key();
            let expired_by = chrono::Duration::hours(1);
            let token = AccessToken {
                sub: random(),
                exp: Utc::now() - expired_by,
            };
            let encoded = encode(&key, &token).unwrap();
            let err = decode::<AccessToken>(&key, &encoded).unwrap_err();
            assert_eq!(
                err,
                Error::Validation(ValidationError::Expired {
                    seconds: expired_by.num_seconds() as u64
                })
            );
        }

        #[test]
        fn invalid_signature() {
            let valid_key = get_key();
            let invalid_key = get_key();
            let expired_by = chrono::Duration::hours(1);
            let token = AccessToken {
                sub: random(),
                exp: Utc::now() - expired_by,
            };
            let encoded = encode(&valid_key, &token).unwrap();
            let err = decode::<AccessToken>(&invalid_key, &encoded).unwrap_err();
            assert_eq!(err, Error::Decode(DecodeError::InvalidSignature));
        }
    }
}

//
// impl AccessToken {
//     pub fn decode(token: &str, key: &DecodingKey, aud: &UserID) -> Result<TokenData<Self>, Error> {
//         let validation = Validation {
//             sub: Some(aud.to_string()),
//             ..Validation::default()
//         };
//         Ok(jsonwebtoken::decode::<Self>(
//             token.into(),
//             key,
//             &validation,
//         )?)
//     }
//
//     pub fn encode(&self, key: &EncodingKey) -> Result<String, Error> {
//         Ok(jsonwebtoken::encode(&TokenHeader::default(), self, key)?)
//     }
//
//     #[cfg(feature = "actix")]
//     pub fn from_request(
//         req: &actix_web::HttpRequest,
//         key: &DecodingKey,
//     ) -> Result<Self, HeaderError> {
//         Self::decode(token_from_request(req)?)
//     }
// }
//
// impl RefreshToken {
//     pub fn decode<'a>(
//         token: impl Into<&'a str>,
//         key: &DecodingKey,
//         aud: &UserID,
//     ) -> Result<TokenData<Self>, Error> {
//         let validation = Validation {
//             sub: Some(aud.to_string()),
//             ..Validation::default()
//         };
//         jsonwebtoken::decode::<Self>(token.into(), key, &validation).map_err(|err| err.into())
//     }
//
//     pub fn encode(&self, key: &EncodingKey) -> Result<String, Error> {
//         jsonwebtoken::encode(&TokenHeader::default(), self, key)
//     }
// }
//
// mod jwt_numeric_date {
//     //! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
//     use chrono::{DateTime, TimeZone, Utc};
//     use serde::{self, Deserialize, Deserializer, Serializer};
//
//     /// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
//     pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let timestamp = date.timestamp();
//         serializer.serialize_i64(timestamp)
//     }
//
//     /// Attempts to deserialize an i64 and use as a Unix timestamp
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
//             .single() // If there are multiple or no valid DateTimes from timestamp, return None
//             .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::Duration;
//     use rand::random;
//
//     const SECRET: &[u8] = b"some-secret";
//
//     mod access_token {
//         use super::*;
//
//         #[test]
//         fn valid() {
//             let encode_key = EncodingKey::from_secret(SECRET);
//             let decode_key = DecodingKey::from_secret(SECRET);
//
//             let user_id = random::<UserID>();
//             let exp = Utc::now() + Duration::hours(1);
//             let token = AccessToken { sub: user_id, exp };
//             let token = token.encode(&encode_key).unwrap();
//             let token_data = AccessToken::decode(&token, &decode_key, &user_id);
//         }
//     }
// }
