use crate::{
    DecodeError, Decoder, Encoder, ExpirationDate, Payload, Signature, TokenID, VerifyError,
};
use houseflow_types::{UserAgent, UserID};

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

    pub fn from_str(s: &str) -> Result<Self, DecodeError> {
        std::str::FromStr::from_str(s)
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
            user_agent: user_agent.clone(),
            user_id: user_id.clone(),
            expires_at,
        };
        let signature = payload.sign(key);
        Token::new(payload, signature)
    }

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

    pub fn new_access_token(
        key: impl AsRef<[u8]>,
        user_id: &UserID,
        user_agent: &UserAgent,
    ) -> Token {
        Self::new_token(
            key,
            user_id,
            user_agent,
            user_agent.access_token_duration(),
        )
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
        let s = base64::decode(s)?;
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
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

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
                let result = Token::from_str(value).map_err(|err| match err {
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
