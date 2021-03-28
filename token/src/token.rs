use std::convert::TryInto;
use std::time::SystemTime;
use crate::{Signature, Payload, SizedFrame, Error, HmacSha256};
use hmac::{Mac, NewMac};


pub struct Token {
    pub payload: Payload,
    pub signature: Signature,
}

impl Token {
    pub fn from_base64(base64: &[u8]) -> Result<Self, Error> {
        let bytes: &[u8] = &base64::decode(base64)?;

        let bytes: [u8; Self::SIZE] = bytes
            .try_into()
            .map_err(|_| Error::InvalidSize(bytes.len()))?;

        Ok(Self::from_bytes(bytes))

    }
    pub fn from_bytes(b: [u8; Self::SIZE]) -> Self {
        Self {
            signature: b[0 .. Signature::SIZE].try_into().unwrap(),
            payload: Payload::from_bytes(b[Signature::SIZE .. Self::SIZE].try_into().unwrap())
        }
    }

    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        let payload_bytes = self.payload.to_bytes();
        let vector = [&self.signature[..], &payload_bytes[..]].concat();

        vector.try_into().unwrap()
    }

    pub fn verify(&self, key: &[u8]) -> Result<(), Error> {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if self.payload.expires_at < ts {
            return Err(Error::Expired{
                expired_by: ts - self.payload.expires_at,
            });
        }

        let mut mac = HmacSha256::new_varkey(key)
            .expect(format!("Invalid HMAC Key size of {}", key.len()).as_str());

        mac.update(&self.payload.to_bytes());
        mac.verify(&self.signature)
            .map_err(|_err| Error::InvalidSignature)?;

        Ok(())
    }
}

impl SizedFrame for Token {
    const SIZE: usize = Signature::SIZE + Payload::SIZE;
}
