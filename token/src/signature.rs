use crate::{Error, SizedFrame};
use bytes::{Buf, BufMut};
use std::convert::{TryFrom, TryInto};

const SIGNATURE_SIZE: usize = 32; // SHA256 bytes

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    inner: [u8; SIGNATURE_SIZE],
}
impl SizedFrame for Signature {
    const SIZE: usize = SIGNATURE_SIZE;
}

impl Signature {
    pub fn from_buf(buf: &mut impl Buf) -> Result<Self, Error> {
        if buf.remaining() < Self::SIZE {
            return Err(Error::InvalidSize(buf.remaining()));
        }

        let mut inner = [0; SIGNATURE_SIZE];
        buf.copy_to_slice(&mut inner);
        Ok(Self { inner })
    }
    pub fn to_buf(&self, buf: &mut impl BufMut) {
        buf.put_slice(&self.inner);
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = ();

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: bytes.try_into().map_err(|_| ())?,
        })
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}
