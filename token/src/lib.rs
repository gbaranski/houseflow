mod payload;
mod signature;
mod token;

pub use payload::*;
pub use signature::*;
pub use token::*;

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("expected size: `{expected}`, received: `{received}`")]
    InvalidLength { expected: usize, received: usize },

    #[error("received invalid timestamp: `{0}`")]
    InvalidTimestamp(u64),

    #[error("received invalid TokenID: `{0}`")]
    InvalidTokenID(houseflow_types::CredentialError),

    #[error("received invalid UserID: `{0}`")]
    InvalidUserID(houseflow_types::CredentialError),

    #[error("unknown user agent: `{0}`")]
    UnknownUserAgent(u8),
}

pub use houseflow_types::UserAgent;

#[derive(Debug, thiserror::Error)]
pub enum VerifyError {
    #[error("token has expired by `{by:?}`")]
    Expired { by: std::time::Duration },

    #[error("invalid user agent, expected: `{expected}`, received: `{received}`")]
    InvalidUserAgent {
        expected: UserAgent,
        received: UserAgent,
    },

    #[error("invald signature")]
    InvalidSignature,
}

#[derive(Debug, thiserror::Error)]
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
    use rand::random;
    use std::time::{Duration, SystemTime};
    const KEY: &[u8] = b"some hmac key";

    #[test]
    fn sign_verify() {
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
            .verify(KEY, &user_agent)
            .expect("failed token verification");
    }

    #[test]
    fn sign_verify_invalid_signature() {
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
        let result = token.verify(b"some other key", &user_agent).unwrap_err();
        match result {
            VerifyError::InvalidSignature => (),
            _ => panic!("received unexpected error: {}", result),
        }
    }

    #[test]
    fn sign_verify_expired() {
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
        let result = token.verify(KEY, &user_agent).unwrap_err();
        match result {
            VerifyError::Expired { .. } => (),
            _ => panic!("received unexpected error: {}", result),
        };
    }

    #[test]
    fn sign_verify_invalid_user_agent() {
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
        let result = token.verify(KEY, &expected_user_agent).unwrap_err();
        match result {
            VerifyError::InvalidUserAgent { received, expected } => {
                assert_eq!(expected, expected_user_agent);
                assert_eq!(received, received_user_agent);
            }
            _ => panic!("received unexpected error: {}", result),
        };
    }

        #[test]
        fn convert_invalid() {
            let mut buf = BytesMut::with_capacity(Token::SIZE);
            let payload = Payload {
                id: random(),
                user_agent: random(),
                user_id: random(),
                expires_at: SystemTime::now()
                    .checked_add(Duration::from_secs(5))
                    .unwrap().into(),
            };
            let signature = payload.sign(KEY);
            let token = Token::new(payload, signature);
            token.encode(&mut buf);
            buf = buf[0..Token::SIZE - 5].into(); // Malform the data on intention
            Token::decode(&mut buf).unwrap_err();
        }

        #[test]
        fn to_from_bytes_conversion() {
            let mut buf = BytesMut::with_capacity(Token::SIZE);
            let payload = Payload {
                id: random(),
                user_agent: random(),
                user_id: random(),
                expires_at: SystemTime::now()
                    .checked_add(Duration::from_secs(5))
                    .unwrap().into(),
            };
            let signature = payload.clone().sign(KEY);
            let token = Token::new(payload.clone(), signature);
            token.encode(&mut buf);
            let parsed_token = Token::decode(&mut buf).expect("fail decoding token from buf");
            assert_eq!(parsed_token, token);
            parsed_token
                .verify(KEY, &payload.user_agent)
                .expect("Failed veryfing token after bytes conversion");
        }
}
