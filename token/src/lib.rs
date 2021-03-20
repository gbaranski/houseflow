#[derive(Debug)]
pub enum Error {
    InvalidSignature,
    Expired{
        expired_by: u64,
    },
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Expired{expired_by} => write!(f, "token has expired by `{} seconds`", expired_by),
            Error::InvalidSignature => write!(f, "token has invalid signature"),
        }
    }
}

use hmac::Hmac;
use sha2::Sha256;
pub(crate) type HmacSha256 = Hmac<Sha256>;

mod token;
mod payload;
mod signature;

pub use payload::Payload;
pub use signature::Signature;
pub use token::Token;

pub trait SizedFrame {
    const SIZE: usize;
}


#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use super::*;

    const KEY: &[u8] = b"some hmac key";
    const AUDIENCE: [u8; 36] = *b"00000000-0000-0000-0000-000000000000";

    fn get_ts() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    #[test]
    fn sign_verify() {
        let payload = Payload {
            audience: AUDIENCE,
            expires_at: get_ts() + 1000,
        };
        let signed_token = payload.sign(KEY);

        let token = Token {
            payload,
            signature: signed_token.signature,
        };
        let verify = token.verify(KEY);

        assert!(verify.is_ok(), "fail verifying token: {}", verify.unwrap_err());
    }

    #[test]
    fn sign_verify_expired() {
        let payload = Payload {
            audience: AUDIENCE,
            expires_at: get_ts() - 1000,
        };
        let signed_token = payload.sign(KEY);
        let token = Token {
            payload,
            signature: signed_token.signature,
        };

        let verify = token.verify(KEY);

        assert!(verify.is_err(), "fail verifying token, expected to fail because expired");
    }


    #[test]
    fn to_from_bytes_conversion() {
        let payload = Payload {
            audience: AUDIENCE,
            expires_at: get_ts(),
        };
        let signed_token = payload.sign(KEY);
        let token = Token {
            payload,
            signature: signed_token.signature,
        };

        let token = Token::from_bytes(token.to_bytes());
        let verify = token.verify(KEY);

        assert!(verify.is_ok(), "fail verifying token after bytes conversions: {}", verify.unwrap_err());
    }


}

