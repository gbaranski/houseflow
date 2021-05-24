use crate::DecodeError;
use std::time::{Duration, SystemTime};

use crate::{Decoder, Encoder};

#[derive(Debug, Clone, Eq)]
pub struct ExpirationDate {
    inner: SystemTime,
}

impl PartialEq for ExpirationDate {
    fn eq(&self, other: &Self) -> bool {
        self.unix_timestamp().as_secs() == other.unix_timestamp().as_secs()
    }
}

impl Decoder for ExpirationDate {
    const SIZE: usize = std::mem::size_of::<u64>();

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

        let timestamp = buf.get_u64();
        let timestamp = Duration::from_secs(timestamp);
        let system_time = SystemTime::UNIX_EPOCH
            .checked_add(timestamp)
            .ok_or_else(|| DecodeError::InvalidTimestamp(timestamp.as_secs()))?;

        let expiration_date = Self { inner: system_time };

        Ok(expiration_date)
    }
}

impl Encoder for ExpirationDate {
    fn encode(&self, buf: &mut impl bytes::BufMut) {
        buf.put_u64(
            self.inner
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }
}

impl From<SystemTime> for ExpirationDate {
    fn from(v: SystemTime) -> Self {
        Self { inner: v }
    }
}

impl ExpirationDate {
    pub fn unix_timestamp(&self) -> Duration {
        self.inner.duration_since(SystemTime::UNIX_EPOCH).unwrap()
    }

    pub fn has_expired(&self) -> bool {
        self.inner.elapsed().is_ok()
    }
}
