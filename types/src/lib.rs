mod common;
mod device;
mod user;

#[cfg(feature = "admin")]
pub mod admin;

#[cfg(feature = "auth")]
pub mod auth;

#[cfg(feature = "fulfillment")]
pub mod fulfillment;

#[cfg(feature = "lighthouse")]
pub mod lighthouse;

#[cfg(feature = "token")]
pub mod token;

pub use common::*;
pub use device::*;
pub use user::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ResultUntagged<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ResultTagged<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> ResultTagged<T, E> {
    pub fn into_result(self) -> Result<T, E> {
        match self {
            Self::Ok(v) => Ok(v),
            Self::Err(v) => Err(v),
        }
    }
}

impl<T, E> ResultUntagged<T, E> {
    pub fn into_result(self) -> Result<T, E> {
        match self {
            Self::Ok(v) => Ok(v),
            Self::Err(v) => Err(v),
        }
    }
}
