use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};
use thiserror::Error;

pub type DeviceID = DeviceCredential<16>;
pub type DevicePassword = DeviceCredential<32>;

#[derive(Hash, Eq, PartialEq)]
pub struct DeviceCredential<const N: usize> {
    inner: [u8; N],
}

impl<const N: usize> DeviceCredential<N> {
    pub const SIZE: usize = N;
}

#[derive(Debug, Error)]
pub enum DeviceCredentialError {
    #[error("Invalid size, expected: {expected}, received: {received}")]
    InvalidSize { expected: usize, received: usize },

    #[error("Invalid encoding: {0}")]
    InvalidEncoding(#[from] hex::FromHexError),
}

impl<const N: usize> From<[u8; N]> for DeviceCredential<N> {
    fn from(v: [u8; N]) -> Self {
        Self { inner: v }
    }
}

impl<const N: usize> Into<[u8; N]> for DeviceCredential<N> {
    fn into(self) -> [u8; N] {
        self.inner
    }
}

impl<const N: usize> Default for DeviceCredential<N> {
    fn default() -> Self {
        Self { inner: [0; N] }
    }
}

impl<const N: usize> Into<String> for DeviceCredential<N> {
    fn into(self) -> String {
        hex::encode(self.inner)
    }
}

impl<const N: usize> TryFrom<String> for DeviceCredential<N> {
    type Error = DeviceCredentialError;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        // N * 2 because encoding with hex doubles the size

        if v.len() != N * 2 {
            Err(DeviceCredentialError::InvalidSize {
                expected: N * 2,
                received: v.len(),
            })
        } else {
            Ok(Self {
                inner: hex::decode(v)?.try_into().unwrap(),
            })
        }
    }
}

impl<const N: usize> FromStr for DeviceCredential<N> {
    type Err = DeviceCredentialError;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        // N * 2 because encoding with hex doubles the size

        if v.len() != N * 2 {
            Err(DeviceCredentialError::InvalidSize {
                expected: N * 2,
                received: v.len(),
            })
        } else {
            Ok(Self {
                inner: hex::decode(v)?.try_into().unwrap(),
            })
        }
    }
}

impl<const N: usize> std::fmt::Display for DeviceCredential<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", hex::encode(self.inner))
    }
}

impl<const N: usize> std::fmt::Debug for DeviceCredential<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Inner: `{}`", hex::encode(self.inner))
    }
}

impl<const N: usize> rand::distributions::Distribution<DeviceCredential<N>>
    for rand::distributions::Standard
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> DeviceCredential<N> {
        DeviceCredential {
            inner: (0..N)
                .map(|_| rng.gen())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        }
    }
}
