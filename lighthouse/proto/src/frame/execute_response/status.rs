use crate::{DecodeError, Decoder, Encoder, Framed};
use bytes::{Buf, BufMut};
use lighthouse_macros::decoder;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::mem::size_of;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
#[repr(u8)]
pub enum ExecuteResponseStatus {
    /// Confirm that the command succeeded.
    Success,

    /// Target device is unable to perform the command.
    Error,
}

impl Decoder for ExecuteResponseStatus {
    const MIN_SIZE: usize = size_of::<Self>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let v = buf.get_u8();
        Self::try_from(v).map_err(|_| DecodeError::InvalidField {
            field: std::any::type_name::<Self>(),
        })
    }
}

impl Encoder for ExecuteResponseStatus {
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.clone() as u8);
    }
}

impl<'de> Framed<'de> for ExecuteResponseStatus {}

impl TryFrom<u8> for ExecuteResponseStatus {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, ()> {
        Self::iter().find(|e| e.clone() as u8 == v).ok_or(())
    }
}

impl rand::distributions::Distribution<ExecuteResponseStatus> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ExecuteResponseStatus {
        ExecuteResponseStatus::iter()
            .nth(rng.gen_range(0..ExecuteResponseStatus::iter().len()))
            .unwrap()
    }
}
