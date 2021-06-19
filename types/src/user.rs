use crate::common::Credential;
use std::convert::TryFrom;
use strum::{EnumIter, IntoEnumIterator};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type UserID = Credential<16>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, strum::Display)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename = "snake_case"))]
#[repr(u8)]
pub enum UserAgent {
    Internal,
    GoogleSmartHome,
}

use std::time::Duration;
impl UserAgent {
    pub fn refresh_token_duration(&self) -> Option<Duration> {
        match *self {
            Self::Internal => Some(Duration::from_secs(3600 * 24 * 7)), // One week
            Self::GoogleSmartHome => None,                              // Never
        }
    }

    pub fn access_token_duration(&self) -> Option<Duration> {
        match *self {
            Self::Internal => Some(Duration::from_secs(60 * 10)), // 10 Minutes
            Self::GoogleSmartHome => Some(Duration::from_secs(60 * 10)), // 10 Minutes
        }
    }
}

impl TryFrom<u8> for UserAgent {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Self::iter().find(|e| *e as u8 == v).ok_or(())
    }
}

impl rand::distributions::Distribution<UserAgent> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> UserAgent {
        UserAgent::try_from(rng.gen_range(0..UserAgent::iter().len() as u8)).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct User {
    /// Unique ID of the user
    pub id: UserID,

    /// Name of the user
    pub username: String,

    /// Email of the user
    pub email: String,

    /// Hashed user password
    pub password_hash: String,
}
