use crate::common::Credential;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

pub type UserID = Credential<16>;

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

