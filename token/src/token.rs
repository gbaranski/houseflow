use crate::{DecodeError, Decoder, Encoder, Payload, Signature, VerifyError};
use houseflow_types::UserAgent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub payload: Payload,
    pub signature: Signature,
}

impl Token {
    pub const BASE64_SIZE: usize = ((4 * Self::SIZE / 3) + 3) & !3;

    pub fn new(payload: Payload, signature: Signature) -> Self {
        Self { payload, signature }
    }

    pub fn from_base64(value: impl AsRef<str>) -> Result<Self, DecodeError> {
        use bytes::BytesMut;
        let value = value.as_ref();

        if value.len() != Self::BASE64_SIZE {
            return Err(DecodeError::InvalidLength {
                expected: Self::BASE64_SIZE,
                received: value.len(),
            });
        }

        let value = base64::decode(value)?;
        let mut buf = BytesMut::from(value.as_slice());
        Self::decode(&mut buf)
    }

    pub fn into_base64(self) -> String {
        use bytes::BytesMut;
        let mut buf = BytesMut::with_capacity(Self::SIZE);
        self.encode(&mut buf);
        base64::encode(buf)
    }

    pub fn base64(&self) -> String {
        use bytes::BytesMut;
        let mut buf = BytesMut::with_capacity(Self::SIZE);
        self.encode(&mut buf);
        base64::encode(buf)
    }

    pub fn verify(&self, key: impl AsRef<[u8]>, user_agent: &UserAgent) -> Result<(), VerifyError> {
        self.payload.verify(user_agent)?;
        self.signature.verify(&self.payload, key)?;
        Ok(())
    }

    #[inline]
    pub fn has_expired(&self) -> bool {
        self.payload.expires_at.has_expired()
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

#[cfg(feature = "serde")]
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[cfg(feature = "serde")]
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
                let result = Token::from_base64(value).map_err(|err| match err {
                    DecodeError::InvalidBase64Encoding(_) => {
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
                });
                Ok(result?)
            }
        }

        deserializer.deserialize_str(TokenVisitor)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let base64 = self.base64();
        serializer.serialize_str(&base64)
    }
}

// use crate::{Error, HmacSha256, Payload, Signature, SizedFrame};
// use bytes::{Buf, BufMut, BytesMut};
// use hmac::{Mac, NewMac};
// use houseflow_types::UserAgent;
// use std::time::SystemTime;
//
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Token {
//     pub signature: Signature,
//     pub payload: Payload,
// }
//
// impl SizedFrame for Token {
//     const SIZE: usize = Signature::SIZE + Payload::SIZE;
// }
//
// impl Token {
//     #[cfg(feature = "serde")]
//     const BASE64_SIZE: usize = (Self::SIZE / 3) * 4;
//
//     pub fn from_base64(base64: impl AsRef<[u8]>) -> Result<Self, Error> {
//         let mut bytes: &[u8] = &base64::decode(base64)?;
//         Self::from_buf(&mut bytes)
//     }
//
//     pub fn to_base64(&self) -> String {
//         let mut bytes = BytesMut::with_capacity(Self::SIZE);
//         self.signature.to_buf(&mut bytes);
//         self.payload.to_buf(&mut bytes);
//         base64::encode(bytes)
//     }
//
//     pub fn from_buf(buf: &mut impl Buf) -> Result<Self, Error> {
//         let signature = Signature::from_buf(buf)?;
//         let payload = Payload::from_buf(buf)?;
//         Ok(Self { payload, signature })
//     }
//
//     pub fn to_buf(&self, buf: &mut impl BufMut) {
//         self.signature.to_buf(buf);
//         self.payload.to_buf(buf);
//     }
//
//     pub fn has_expired(&self) -> bool {
//         self.payload.expires_at.elapsed().is_ok()
//     }
//
//     pub fn verify(&self, key: &[u8], user_agent: &UserAgent) -> Result<(), Error> {
//         if self.payload.user_agent != *user_agent {
//             return Err(Error::InvalidUserAgent {
//                 expected: self.payload.user_agent,
//                 received: *user_agent,
//             });
//         }
//         if self.has_expired() {
//             return Err(Error::Expired {
//                 expired_by: self
//                     .payload
//                     .expires_at
//                     .duration_since(SystemTime::UNIX_EPOCH)
//                     .unwrap()
//                     .as_secs(),
//             });
//         }
//         let mut mac = HmacSha256::new_varkey(key)
//             .expect(format!("Invalid HMAC Key size of {}", key.len()).as_str());
//
//         let mut payload_buf = BytesMut::with_capacity(Payload::SIZE);
//         self.payload.to_buf(&mut payload_buf);
//         mac.update(&payload_buf);
//         mac.verify(self.signature.as_ref())
//             .map_err(|_err| Error::InvalidSignature)?;
//
//         Ok(())
//     }
// }
//
