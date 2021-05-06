use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};
use bytes::Buf;
use thiserror::Error;

pub type UserID = Credential<32>;
pub type DeviceID = Credential<16>;
pub type DevicePassword = Credential<32>;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Credential<const N: usize> {
    inner: [u8; N],
}

impl<const N: usize> Credential<N> {
    pub const SIZE: usize = N;

    pub fn into_bytes(self) -> [u8; N] {
        self.inner
    }

    pub fn from_bytes(bytes: [u8; N]) -> Self {
        Self::from(bytes)
    }

    pub fn from_buf(buf: &mut impl Buf) -> Self {
        let mut inner = [0; N];
        buf.copy_to_slice(&mut inner);
        Self {
            inner,
        }
    }
}

impl<const N: usize> AsRef<[u8]> for Credential<N> {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

#[derive(Debug, Error)]
pub enum CredentialError {
    #[error("Invalid size, expected: {expected}, received: {received}")]
    InvalidSize { expected: usize, received: usize },

    #[error("Invalid encoding: {0}")]
    InvalidEncoding(#[from] hex::FromHexError),
}

impl<const N: usize> From<[u8; N]> for Credential<N> {
    fn from(v: [u8; N]) -> Self {
        Self { inner: v }
    }
}

impl<const N: usize> Into<[u8; N]> for Credential<N> {
    fn into(self) -> [u8; N] {
        self.inner
    }
}

impl<const N: usize> Default for Credential<N> {
    fn default() -> Self {
        Self { inner: [0; N] }
    }
}

impl<const N: usize> Into<String> for Credential<N> {
    fn into(self) -> String {
        hex::encode(self.inner)
    }
}

impl<const N: usize> TryFrom<String> for Credential<N> {
    type Error = CredentialError;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        // N * 2 because encoding with hex doubles the size

        if v.len() != N * 2 {
            Err(CredentialError::InvalidSize {
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

impl<const N: usize> FromStr for Credential<N> {
    type Err = CredentialError;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        // N * 2 because encoding with hex doubles the size

        if v.len() != N * 2 {
            Err(CredentialError::InvalidSize {
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

impl<const N: usize> std::fmt::Display for Credential<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", hex::encode(self.inner))
    }
}

impl<const N: usize> std::fmt::Debug for Credential<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Inner: `{}`", hex::encode(self.inner))
    }
}

impl<const N: usize> rand::distributions::Distribution<Credential<N>>
    for rand::distributions::Standard
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Credential<N> {
        Credential {
            inner: (0..N)
                .map(|_| rng.gen())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        }
    }
}
