use std::convert::TryInto;
use crate::{SizedFrame, Token, HmacSha256};
use hmac::{Mac, NewMac};
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct Payload {
    pub audience: Uuid,
    pub expires_at: u64,
}

const UUID_SIZE: usize = 16;

impl SizedFrame for Payload {
    // 8 first bytes are unsigned 64 bit integer
    const SIZE: usize = 8 + UUID_SIZE;
}

impl Payload {
    pub fn from_bytes(b: [u8; Self::SIZE]) -> Self {
        let uuid_bytes: uuid::Bytes = b[8 .. 8 + UUID_SIZE]
            .try_into()
            .unwrap();

        Self {
            expires_at: u64::from_be_bytes(b[0 .. 8].try_into().unwrap()),
            audience: Uuid::from_bytes(uuid_bytes),
        }
    }

    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        let expires_at_bytes = self.expires_at.to_be_bytes();
        let vector = [&expires_at_bytes[..], &self.audience.as_bytes()[..]].concat();

        vector.try_into().unwrap()
    }

    pub fn sign(self, key: &[u8]) -> Token {
        let mut mac = HmacSha256::new_varkey(key)
            .expect(format!("Invalid HMAC Key size of {}", key.len()).as_str());

        mac.update(&self.to_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes().to_owned();

        Token {
            payload: self.clone(),
            signature: code_bytes.try_into().unwrap(),
        }
    }
}

