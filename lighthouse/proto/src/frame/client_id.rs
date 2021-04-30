use std::convert::{TryFrom, TryInto};

pub const CLIENT_ID_SIZE: usize = 16;
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientID {
    inner: [u8; CLIENT_ID_SIZE],
}



impl From<[u8; 16]> for ClientID {
    fn from(item: [u8; 16]) -> Self {
        Self { inner: item }
    }
}

impl Into<[u8; 16]> for ClientID {
    fn into(self) -> [u8; 16] {
        self.inner
    }
}

impl Default for ClientID {
    fn default() -> Self {
        Self { inner: [0; 16] }
    }
}

impl Into<String> for ClientID {
    fn into(self) -> String {
        hex::encode(self.inner)
    }
}

impl TryFrom<String> for ClientID {
    type Error = Box<dyn std::error::Error>;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        let bytes = hex::decode(v)?;
        Ok(Self {
            inner: bytes.try_into().map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid size")
            })?,
        })
    }
}

impl<'a> TryFrom<&'a str> for ClientID {
    type Error = Box<dyn std::error::Error>;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        let bytes = hex::decode(v)?;

        Ok(Self {
            inner: bytes.try_into().map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid size")
            })?,
        })
    }
}

impl std::fmt::Display for ClientID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Debug for ClientID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ClientID: `{}`", self.to_string())
    }
}

impl rand::distributions::Distribution<ClientID> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ClientID {
        ClientID { inner: rng.gen() }
    }
}
