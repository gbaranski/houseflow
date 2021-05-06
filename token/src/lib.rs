use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid Base64 encoding: {0} ")]
    InvalidBase64Encoding(#[from] base64::DecodeError),

    #[error("Invalid token size: {0}")]
    InvalidSize(usize),

    #[error("Invalid token signature")]
    InvalidSignature,

    #[error("Malformed payload {0:?}")]
    MalformedPayload(Option<Box<dyn std::error::Error>>),

    #[error("Token is expired by {expired_by}s")]
    Expired { expired_by: u64 },
}

use hmac::Hmac;
use sha2::Sha256;
pub(crate) type HmacSha256 = Hmac<Sha256>;

mod payload;
mod signature;
mod token;

pub use payload::Payload;
pub use signature::Signature;
pub use token::Token;

pub trait SizedFrame {
    const SIZE: usize;
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
        let payload = Payload {
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_add(Duration::from_secs(5))
                .unwrap(),
        };
        let token = payload.sign(KEY);
        token.verify(KEY).expect("failed token verification");
    }

    #[test]
    fn sign_verify_invalid_signature() {
        let payload = Payload {
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_sub(Duration::from_secs(5))
                .unwrap(),
        };
        let token = payload.sign(KEY);
        token.verify(b"some other key").expect_err("failed token verification");
    }

    #[test]
    fn sign_verify_expired() {
        let payload = Payload {
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_sub(Duration::from_secs(5))
                .unwrap(),
        };
        let token = payload.sign(KEY);
        token.verify(KEY).expect_err("failed token verification");
    }

    #[test]
    fn convert_invalid() {
        let mut buf = BytesMut::with_capacity(Token::SIZE);
        let payload = Payload {
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_add(Duration::from_secs(5))
                .unwrap(),
        };
        let token = payload.sign(KEY);
        token.to_buf(&mut buf);
        buf = buf[0..Token::SIZE - 5].into(); // Malform the data on intention

        Token::from_buf(&mut buf)
            .expect_err("reading token from buffer succeded even if it should not succeed");
    }

    #[test]
    fn to_from_bytes_conversion() {
        let mut buf = BytesMut::with_capacity(Token::SIZE);
        let payload = Payload {
            user_id: random(),
            expires_at: SystemTime::now()
                .checked_add(Duration::from_secs(5))
                .unwrap(),
        };
        let token = payload.clone().sign(KEY);
        token.to_buf(&mut buf);

        let parsed_token = Token::from_buf(&mut buf).expect("failed reading token from buffer");
        assert_eq!(parsed_token, token);
        parsed_token
            .verify(KEY)
            .expect("Failed veryfing token after bytes conversion");
    }
}
