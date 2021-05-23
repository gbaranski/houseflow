use crate::{Error, HmacSha256, Payload, Signature, SizedFrame};
use bytes::{Buf, BufMut, BytesMut};
use hmac::{Mac, NewMac};
use houseflow_types::UserAgent;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub signature: Signature,
    pub payload: Payload,
}

impl SizedFrame for Token {
    const SIZE: usize = Signature::SIZE + Payload::SIZE;
}

impl Token {
    pub fn from_base64(base64: impl AsRef<[u8]>) -> Result<Self, Error> {
        let mut bytes: &[u8] = &base64::decode(base64)?;
        Self::from_buf(&mut bytes)
    }

    pub fn from_buf(buf: &mut impl Buf) -> Result<Self, Error> {
        let signature = Signature::from_buf(buf)?;
        let payload = Payload::from_buf(buf)?;
        Ok(Self { payload, signature })
    }
    pub fn to_buf(&self, buf: &mut impl BufMut) {
        self.signature.to_buf(buf);
        self.payload.to_buf(buf);
    }

    pub fn verify(&self, key: &[u8], user_agent: &UserAgent) -> Result<(), Error> {
        if self.payload.user_agent != *user_agent {
            return Err(Error::InvalidAgent {
                expected: *user_agent,
                received: self.payload.user_agent,
            });
        }
        if self.payload.expires_at.elapsed().is_ok() {
            return Err(Error::Expired {
                expired_by: self
                    .payload
                    .expires_at
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            });
        }
        let mut mac = HmacSha256::new_varkey(key)
            .expect(format!("Invalid HMAC Key size of {}", key.len()).as_str());

        let mut payload_buf = BytesMut::with_capacity(Payload::SIZE);
        self.payload.to_buf(&mut payload_buf);
        mac.update(&payload_buf);
        mac.verify(self.signature.as_ref())
            .map_err(|_err| Error::InvalidSignature)?;

        Ok(())
    }
}
