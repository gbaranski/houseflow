use crate::{Error, HmacSha256, SizedFrame, Token};
use bytes::{Buf, BufMut, BytesMut};
use hmac::{Mac, NewMac};
use houseflow_types::UserID;
use std::{
    convert::TryInto,
    time::{Duration, SystemTime},
};

#[derive(Debug, Clone, Eq)]
pub struct Payload {
    pub user_id: UserID,
    pub expires_at: SystemTime,
}

impl PartialEq for Payload {
    fn eq(&self, cmp: &Self) -> bool {
        self.user_id == cmp.user_id
            && self
                .expires_at
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                == cmp
                    .expires_at
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
    }
}

const UNIX_TIMESTAMP_SECONDS_SIZE: usize = 8;

impl SizedFrame for Payload {
    // 8 first bytes are unsigned 64 bit integer
    const SIZE: usize = UNIX_TIMESTAMP_SECONDS_SIZE + UserID::SIZE;
}

impl Payload {
    pub fn from_buf(buf: &mut impl Buf) -> Result<Self, Error> {
        if buf.remaining() < Self::SIZE {
            return Err(Error::InvalidSize(buf.remaining()))
        }

        let expires_at = SystemTime::UNIX_EPOCH
            .checked_add(Duration::from_secs(buf.get_u64()))
            .unwrap();
        let user_id = UserID::from_buf(buf);
        Ok(Self {
            expires_at,
            user_id,
        })
    }

    pub fn to_buf(&self, buf: &mut impl BufMut) {
        buf.put_u64(
            self.expires_at
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        buf.put_slice(self.user_id.as_ref());
    }

    pub fn sign(self, key: &[u8]) -> Token {
        let mut mac = HmacSha256::new_varkey(key)
            .expect(format!("Invalid HMAC Key size of {}", key.len()).as_str());

        let mut self_bytes = BytesMut::with_capacity(Self::SIZE);
        self.to_buf(&mut self_bytes);
        mac.update(&self_bytes);

        let result = mac.finalize();
        let code_bytes: &[u8] = &result.into_bytes();

        Token {
            payload: self,
            signature: code_bytes.try_into().unwrap(),
        }
    }
}
