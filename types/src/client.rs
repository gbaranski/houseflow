use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum::Display, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[repr(u8)]
pub enum Client {
    Internal,
    GoogleHome,
}

#[cfg(feature = "token")]
use chrono::Duration;

#[cfg(feature = "token")]
impl Client {
    pub fn refresh_token_duration(&self) -> Option<Duration> {
        match *self {
            Self::Internal => Some(Duration::days(7)),
            Self::GoogleHome => None,
        }
    }

    pub fn access_token_duration(&self) -> Duration {
        match *self {
            Self::Internal => Duration::minutes(10),
            Self::GoogleHome => Duration::minutes(10),
        }
    }
}
