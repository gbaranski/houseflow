use crate::common::Credential;
use std::convert::TryFrom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type UserID = Credential<16>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumIter, strum_macros::Display)]
#[repr(u8)]
pub enum UserAgent {
    Internal,
    GoogleSmartHome,
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

    /// First name of the user
    pub first_name: String,

    /// Last name of the user
    pub last_name: String,

    /// Email of the user
    pub email: String,

    /// Hashed user password
    pub password_hash: String,
}
