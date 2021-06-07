use crate::{DecodeError, Decoder, Encoder, Signature, VerifyError};
use types::{UserAgent, UserID};

pub type TokenID = types::Credential<16>;
mod exp_date;
mod user_agent;
pub use exp_date::ExpirationDate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payload {
    pub id: TokenID,
    pub user_agent: UserAgent,
    pub user_id: UserID,
    pub expires_at: ExpirationDate,
}

impl Decoder for Payload {
    const SIZE: usize = TokenID::SIZE + UserAgent::SIZE + UserID::SIZE + ExpirationDate::SIZE;

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
        let token_id = TokenID::decode(buf).map_err(|err| DecodeError::InvalidTokenID(err))?;
        let user_id = UserID::decode(buf).map_err(|err| DecodeError::InvalidUserID(err))?;
        let user_agent = UserAgent::decode(buf)?;
        let exp_date = ExpirationDate::decode(buf)?;

        Ok(Self {
            id: token_id,
            expires_at: exp_date,
            user_agent,
            user_id,
        })
    }
}

impl Encoder for Payload {
    fn encode(&self, buf: &mut impl bytes::BufMut) {
        self.id.encode(buf);
        self.user_id.encode(buf);
        self.user_agent.encode(buf);
        self.expires_at.encode(buf);
    }
}

use bytes::BytesMut;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

impl Payload {
    pub fn new(user_agent: UserAgent, user_id: UserID, expires_at: ExpirationDate) -> Self {
        Self {
            id: rand::random(),
            user_agent,
            user_id,
            expires_at,
        }
    }

    pub fn sign(&self, key: impl AsRef<[u8]>) -> Signature {
        let mut mac = HmacSha256::new_from_slice(key.as_ref()).unwrap();
        let mut bytes = BytesMut::with_capacity(Payload::SIZE);
        self.encode(&mut bytes);
        mac.update(&bytes);
        let result = mac.finalize();
        Signature::new(result)
    }

    #[inline]
    pub fn verify_user_agent(&self, user_agent: Option<&UserAgent>) -> Result<(), VerifyError> {
        match user_agent {
            Some(user_agent) if self.user_agent != *user_agent => {
                Err(VerifyError::InvalidUserAgent {
                    expected: user_agent.clone(),
                    received: self.user_agent,
                })
            }
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    #[inline]
    pub fn verify_expires_at(&self) -> Result<(), VerifyError> {
        if self.expires_at.has_expired() {
            Err(VerifyError::Expired {
                date: self.expires_at.clone(),
            })
        } else {
            Ok(())
        }
    }

    pub fn verify(&self, user_agent: Option<&UserAgent>) -> Result<(), VerifyError> {
        self.verify_user_agent(user_agent)?;
        self.verify_expires_at()?;
        Ok(())
    }
}
