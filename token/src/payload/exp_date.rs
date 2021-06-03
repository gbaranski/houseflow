use crate::DecodeError;
use std::time::{Duration, SystemTime};

use crate::{Decoder, Encoder};

#[derive(Debug, Clone, Eq)]
pub struct ExpirationDate {
    system_time: Option<SystemTime>,
}

impl std::fmt::Display for ExpirationDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self.system_time {
            Some(system_time) => system_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            None => "never".to_string(),
        };
        write!(f, "{}", msg)
    }
}

impl PartialEq for ExpirationDate {
    fn eq(&self, other: &Self) -> bool {
        self.unix_timestamp().map(|v| v.as_secs()) == other.unix_timestamp().map(|v| v.as_secs())
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
        let expiration_date = match timestamp {
            0 => Self { system_time: None },
            timestamp @ _ => {
                let timestamp = Duration::from_secs(timestamp);
                let system_time = SystemTime::UNIX_EPOCH
                    .checked_add(timestamp)
                    .ok_or_else(|| DecodeError::InvalidTimestamp(timestamp.as_secs()))?;

                Self {
                    system_time: Some(system_time),
                }
            }
        };

        Ok(expiration_date)
    }
}

impl Encoder for ExpirationDate {
    fn encode(&self, buf: &mut impl bytes::BufMut) {
        let v = match self.system_time {
            Some(system_time) => system_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            None => 0,
        };
        buf.put_u64(v)
    }
}

impl From<SystemTime> for ExpirationDate {
    fn from(value: SystemTime) -> Self {
        Self {
            system_time: Some(value),
        }
    }
}

impl ExpirationDate {
    pub fn unix_timestamp(&self) -> Option<Duration> {
        match self.system_time {
            Some(system_time) => Some(system_time.duration_since(SystemTime::UNIX_EPOCH).unwrap()),
            None => None,
        }
    }

    pub fn has_expired(&self) -> bool {
        match self.system_time {
            Some(system_time) => system_time.elapsed().is_ok(),
            None => false,
        }
    }

    pub fn from_duration(duration: Option<Duration>) -> Self {
        match duration {
            Some(duration) => SystemTime::now().checked_add(duration).unwrap().into(),
            None => Self { system_time: None },
        }
    }
}
