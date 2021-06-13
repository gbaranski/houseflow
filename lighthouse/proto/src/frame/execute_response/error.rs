use crate::{DecodeError, Decoder, Encoder, Framed};
use bytes::{Buf, BufMut};
use lighthouse_macros::decoder;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::mem::size_of;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
#[repr(u16)]
pub enum ExecuteResponseError {
    /// No error occurred
    None = 0x0000,

    /// Actually, <device(s)> <doesn't/don't> support that functionality.
    FunctionNotSupported = 0x0001,
}

impl Decoder for ExecuteResponseError {
    const MIN_SIZE: usize = size_of::<Self>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let v = buf.get_u16();
        Self::try_from(v).map_err(|_| DecodeError::InvalidField {
            field: std::any::type_name::<Self>(),
        })
    }
}

impl Encoder for ExecuteResponseError {
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u16(self.clone() as u16);
    }
}

impl<'de> Framed<'de> for ExecuteResponseError {}

impl TryFrom<u16> for ExecuteResponseError {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Self::iter().find(|e| e.clone() as u16 == v).ok_or(())
    }
}

impl rand::distributions::Distribution<ExecuteResponseError> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ExecuteResponseError {
        ExecuteResponseError::iter()
            .nth(rng.gen_range(0..ExecuteResponseError::iter().len()))
            .unwrap()
    }
}
