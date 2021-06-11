mod payload;
mod signature;
mod token;

#[cfg(feature = "store")]
pub mod store;

pub use payload::*;
pub use signature::*;
pub use token::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DecodeError {
    #[error("expected size: `{expected}`, received: `{received}`")]
    InvalidLength { expected: usize, received: usize },

    #[error("received invalid timestamp: `{0}`")]
    InvalidTimestamp(u64),

    #[error("received invalid TokenID: `{0}`")]
    InvalidTokenID(types::CredentialError),

    #[error("received invalid UserID: `{0}`")]
    InvalidUserID(types::CredentialError),

    #[error("unknown user agent: `{0}`")]
    UnknownUserAgent(u8),

    #[error("invalid encoding: `{0}`")]
    InvalidEncoding(String),
}


#[cfg(feature = "actix")]
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

pub use types::UserAgent;

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

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    use types::UserID;
    use rand::random;
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
