mod payload;
mod signature;

pub use payload::*;
pub use signature::*;

#[cfg(any(test, feature = "serde"))]
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DecodeError {
    #[error("expected size: `{expected}`, received: `{received}`")]
    InvalidLength { expected: usize, received: usize },

    #[error("received invalid timestamp: `{0}`")]
    InvalidTimestamp(u64),

    #[error("received invalid TokenID: `{0}`")]
    InvalidTokenID(crate::CredentialError),

    #[error("received invalid UserID: `{0}`")]
    InvalidUserID(crate::CredentialError),

    #[error("unknown user agent: `{0}`")]
    UnknownUserAgent(u8),

    #[error("invalid encoding: `{0}`")]
    InvalidEncoding(String),
}

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DecodeHeaderError {
    #[error("decode error: {0}")]
    DecodeError(#[from] DecodeError),

    #[error("header is missing")]
    MissingHeader,

    #[error("invalid header encoding: {0}")]
    InvalidEncoding(String),

    #[error("invalid header value syntax")]
    InvalidSyntax,

    #[error("invalid header schema: {0}")]
    InvalidSchema(String),
}

use crate::UserAgent;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VerifyError {
    #[error("token is expired since:  `{date}`")]
    Expired { date: ExpirationDate },

    #[error("invalid user agent, expected: `{expected}`, received: `{received}`")]
    InvalidUserAgent {
        expected: UserAgent,
        received: UserAgent,
    },

    #[error("invalid signature")]
    InvalidSignature,
}

#[derive(Debug, Clone, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Error {
    #[error("token failed to verify: `{0}`")]
    VerifyError(#[from] VerifyError),

    #[error("token failed to decode: `{0}`")]
    DecodeError(#[from] DecodeError),
}

use bytes::{Buf, BufMut};

pub trait Decoder {
    const SIZE: usize;
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub trait Encoder {
    fn encode(&self, buf: &mut impl BufMut);
}

use crate::UserID;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    payload: Payload,
    signature: Signature,
}

impl Token {
    pub const BASE64_SIZE: usize = ((4 * Self::SIZE / 3) + 3) & !3;

    pub fn new(payload: Payload, signature: Signature) -> Self {
        Self { payload, signature }
    }

    pub fn verify(
        &self,
        key: impl AsRef<[u8]>,
        user_agent: Option<&UserAgent>,
    ) -> Result<(), VerifyError> {
        self.payload.verify(user_agent)?;
        self.signature.verify(&self.payload, key)?;
        Ok(())
    }

    pub fn has_expired(&self) -> bool {
        self.payload.expires_at.has_expired()
    }

    fn new_token(
        key: impl AsRef<[u8]>,
        user_id: &UserID,
        user_agent: &UserAgent,
        expires_in: Option<std::time::Duration>,
    ) -> Token {
        let expires_at = ExpirationDate::from_duration(expires_in);
        let payload = Payload {
            id: rand::random(),
            user_agent: *user_agent,
            user_id: user_id.clone(),
            expires_at,
        };
        let signature = payload.sign(key);
        Token::new(payload, signature)
    }

    /// Creates new refresh token for given credentials
    pub fn new_refresh_token(
        key: impl AsRef<[u8]>,
        user_id: &UserID,
        user_agent: &UserAgent,
    ) -> Token {
        Self::new_token(
            key,
            user_id,
            user_agent,
            user_agent.refresh_token_duration(),
        )
    }

    /// Creates new access token for given credentials
    pub fn new_access_token(
        key: impl AsRef<[u8]>,
        user_id: &UserID,
        user_agent: &UserAgent,
    ) -> Token {
        Self::new_token(key, user_id, user_agent, user_agent.access_token_duration())
    }

    #[cfg(feature = "actix")]
    pub fn from_request(req: &actix_web::HttpRequest) -> Result<Self, DecodeHeaderError> {
        use std::str::FromStr;
        let header_str = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .ok_or(DecodeHeaderError::MissingHeader)?
            .to_str()
            .map_err(|err| DecodeHeaderError::InvalidEncoding(err.to_string()))?;

        let (schema, token) = header_str
            .split_once(' ')
            .ok_or(DecodeHeaderError::InvalidSyntax)?;
        if schema != "Bearer" {
            return Err(DecodeHeaderError::InvalidSchema(schema.to_string()));
        }
        let token = Token::from_str(token)?;
        Ok(token)
    }

    #[inline]
    pub fn id(&self) -> &TokenID {
        &self.payload.id
    }

    #[inline]
    pub fn user_agent(&self) -> &UserAgent {
        &self.payload.user_agent
    }

    #[inline]
    pub fn user_id(&self) -> &UserID {
        &self.payload.user_id
    }

    #[inline]
    pub fn expires_at(&self) -> &ExpirationDate {
        &self.payload.expires_at
    }
}

impl std::string::ToString for Token {
    fn to_string(&self) -> String {
        use bytes::BytesMut;

        let mut buf = BytesMut::with_capacity(Self::SIZE);
        self.encode(&mut buf);
        base64::encode(buf)
    }
}

impl std::str::FromStr for Token {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use bytes::BytesMut;
        if s.len() != Self::BASE64_SIZE {
            return Err(DecodeError::InvalidLength {
                expected: Self::BASE64_SIZE,
                received: s.len(),
            });
        }
        let s = base64::decode(s).map_err(|err| DecodeError::InvalidEncoding(err.to_string()))?;
        let mut s = BytesMut::from(s.as_slice());
        Self::decode(&mut s)
    }
}

impl Decoder for Token {
    const SIZE: usize = Payload::SIZE + Signature::SIZE;

    fn decode(buf: &mut impl bytes::Buf) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        if buf.remaining() < Self::SIZE {
            return Err(DecodeError::InvalidLength {
                expected: Self::SIZE,
                received: buf.remaining(),
            });
        }
        let payload = Payload::decode(buf)?;
        let signature = Signature::decode(buf)?;
        Ok(Self { payload, signature })
    }
}

impl Encoder for Token {
    fn encode(&self, buf: &mut impl bytes::BufMut) {
        self.payload.encode(buf);
        self.signature.encode(buf);
    }
}

#[cfg(any(test, feature = "serde"))]
impl<'de> Deserialize<'de> for Token {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TokenVisitor;
        impl<'de> Visitor<'de> for TokenVisitor {
            type Value = Token;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(&format!("string of length `{}`", Token::BASE64_SIZE))
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use std::str::FromStr;

                Token::from_str(value).map_err(|err| match err {
                    DecodeError::InvalidEncoding(_) => {
                        de::Error::invalid_value(de::Unexpected::Str(value), &"valid base64 str")
                    }
                    DecodeError::InvalidLength { expected, received } => {
                        de::Error::invalid_length(received, &format!("size: {}", expected).as_str())
                    }
                    DecodeError::InvalidTimestamp(ts) => {
                        de::Error::invalid_value(de::Unexpected::Unsigned(ts), &"valid base64 str")
                    }
                    DecodeError::InvalidTokenID(err) => de::Error::custom(err),
                    DecodeError::InvalidUserID(err) => de::Error::custom(err),
                    DecodeError::UnknownUserAgent(value) => de::Error::invalid_value(
                        de::Unexpected::Unsigned(value as u64),
                        &"valid UserAgent",
                    ),
                })
            }
        }

        deserializer.deserialize_str(TokenVisitor)
    }
}

#[cfg(any(test, feature = "serde"))]
impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = self.to_string();
        serializer.serialize_str(&string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UserID;
    use bytes::BytesMut;
    use rand::random;
    use std::str::FromStr;
    use std::time::{Duration, SystemTime};
    const KEY: &[u8] = b"some hmac key";

    #[test]
    fn test_sign_verify() {
        let user_agent: UserAgent = random();
        let payload = Payload {
            id: random(),
            user_agent: user_agent.clone(),
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_add(Duration::from_secs(5))
                .unwrap()
                .into(),
        };
        let signature = payload.sign(KEY);
        let token = Token::new(payload, signature);
        token
            .verify(KEY, Some(&user_agent))
            .expect("failed token verification");
    }

    #[test]
    fn test_sign_verify_invalid_signature() {
        let user_agent: UserAgent = random();
        let payload = Payload {
            id: random(),
            user_agent: user_agent.clone(),
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_add(Duration::from_secs(5))
                .unwrap()
                .into(),
        };
        let signature = payload.sign(KEY);
        let token = Token::new(payload, signature);
        let result = token
            .verify(b"some other key", Some(&user_agent))
            .unwrap_err();
        match result {
            VerifyError::InvalidSignature => (),
            _ => panic!("received unexpected error: {}", result),
        }
    }

    #[test]
    fn test_sign_verify_expired() {
        let user_agent: UserAgent = random();
        let payload = Payload {
            id: random(),
            user_agent: user_agent.clone(),
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_sub(Duration::from_secs(5))
                .unwrap()
                .into(),
        };
        let signature = payload.sign(KEY);
        let token = Token::new(payload, signature);
        let result = token.verify(KEY, Some(&user_agent)).unwrap_err();
        match result {
            VerifyError::Expired { .. } => (),
            _ => panic!("received unexpected error: {}", result),
        };
    }

    #[test]
    fn test_sign_verify_invalid_user_agent() {
        let expected_user_agent = UserAgent::Internal;
        let received_user_agent = UserAgent::GoogleSmartHome;
        let payload = Payload {
            id: random(),
            user_agent: received_user_agent,
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_sub(Duration::from_secs(5))
                .unwrap()
                .into(),
        };
        let signature = payload.sign(KEY);
        let token = Token::new(payload, signature);
        let result = token.verify(KEY, Some(&expected_user_agent)).unwrap_err();
        match result {
            VerifyError::InvalidUserAgent { received, expected } => {
                assert_eq!(expected, expected_user_agent);
                assert_eq!(received, received_user_agent);
            }
            _ => panic!("received unexpected error: {}", result),
        };
    }

    #[test]
    fn test_convert_invalid() {
        let mut buf = BytesMut::with_capacity(Token::SIZE);
        let payload = Payload {
            id: random(),
            user_agent: random(),
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_add(Duration::from_secs(5))
                .unwrap()
                .into(),
        };
        let signature = payload.sign(KEY);
        let token = Token::new(payload, signature);
        token.encode(&mut buf);
        buf = buf[0..Token::SIZE - 5].into(); // Malform the data on intention
        Token::decode(&mut buf).unwrap_err();
    }

    #[test]
    fn test_formats_integrity() {
        let mut buf = BytesMut::new();
        let payload = Payload {
            id: random(),
            user_agent: random(),
            user_id: random(),
            expires_at: ExpirationDate::from_duration(Some(Duration::from_secs(5))),
        };
        let signature = payload.clone().sign(KEY);
        let token = Token::new(payload.clone(), signature);
        token.encode(&mut buf);
        let stringified = base64::decode(token.to_string()).unwrap();
        assert_eq!(buf, stringified);
    }

    #[test]
    fn test_bytes_conversions() {
        let mut buf = BytesMut::new();
        let payload = Payload {
            id: random(),
            user_agent: random(),
            user_id: random(),
            expires_at: ExpirationDate::from_duration(Some(Duration::from_secs(5))),
        };
        let signature = payload.clone().sign(KEY);
        let token = Token::new(payload.clone(), signature);
        token.encode(&mut buf);
        let token_parsed = Token::decode(&mut buf).unwrap();
        assert_eq!(token, token_parsed);
    }

    #[test]
    fn test_string_conversions() {
        let payload = Payload {
            id: TokenID::from_bytes(*b"abcdefghijklemno"),
            user_agent: UserAgent::GoogleSmartHome,
            user_id: UserID::from_bytes(*b"abcdefghijklemno"),
            expires_at: ExpirationDate::from_duration(None),
        };
        let signature = payload.clone().sign(KEY);
        let token = Token::new(payload.clone(), signature);
        let token_string = token.to_string();
        let token_string_parsed = Token::from_str(&token_string).unwrap();
        assert_eq!(token, token_string_parsed);
    }

    #[test]
    fn test_serde() {
        let user_agent: UserAgent = random();
        let payload = Payload {
            id: random(),
            user_agent: user_agent.clone(),
            user_id: random(),
            expires_at: ExpirationDate::from_duration(Some(Duration::from_secs(5))),
        };
        let signature = payload.sign(KEY);
        let token = Token::new(payload, signature);
        let token_json = serde_json::to_string(&token).unwrap();
        let token_json_parsed: Token = serde_json::from_str(&token_json).unwrap();
        assert_eq!(token, token_json_parsed)
    }
}
