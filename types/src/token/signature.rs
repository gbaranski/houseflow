use crate::token::{DecodeError, Decoder, Encoder, Payload, VerifyError};
use bytes::BytesMut;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

const SIGNATURE_SIZE: usize = 32; // SHA256 bytes

#[derive(Clone, PartialEq, Eq)]
pub struct Signature {
    inner: hmac::crypto_mac::Output<hmac::Hmac<Sha256>>,
}

impl Signature {
    pub fn new(inner: hmac::crypto_mac::Output<hmac::Hmac<Sha256>>) -> Self {
        Self { inner }
    }

    pub fn verify(&self, payload: &Payload, key: impl AsRef<[u8]>) -> Result<(), VerifyError> {
        let mut buf = BytesMut::with_capacity(Payload::SIZE);
        payload.encode(&mut buf);
        let mut mac = HmacSha256::new_from_slice(key.as_ref()).unwrap();
        mac.update(&buf);
        mac.verify(&self.inner.clone().into_bytes())
            .map_err(|_| VerifyError::InvalidSignature)
    }
}

impl std::fmt::Debug for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.inner.clone().into_bytes();
        let hex = hex::encode(bytes);
        write!(f, "{}", hex)
    }
}

impl Decoder for Signature {
    const SIZE: usize = SIGNATURE_SIZE;

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

        let mut bytes = [0; SIGNATURE_SIZE];
        buf.copy_to_slice(&mut bytes);
        let inner = hmac::crypto_mac::Output::new(bytes.into());
        Ok(Self { inner })
    }
}

impl Encoder for Signature {
    fn encode(&self, buf: &mut impl bytes::BufMut) {
        buf.put_slice(&self.inner.clone().into_bytes());
    }
}
