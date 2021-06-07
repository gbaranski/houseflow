use crate::{DecodeError, Decoder, Encoder, Framed};
use bytes::{Buf, BufMut};
use lighthouse_macros::decoder;
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    mem::size_of,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
#[repr(u16)]
pub enum CommandCode {
    NoOperation = 0x0000,
    OnOff = 0x0001,
}

impl<'de> Framed<'de> for CommandCode {}

impl Encoder for CommandCode {
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u16(self.clone() as u16);
    }
}

impl Decoder for CommandCode {
    const MIN_SIZE: usize = size_of::<u16>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        buf.get_u16()
            .try_into()
            .map_err(|_| DecodeError::InvalidField { field: "command" })
    }
}

impl TryFrom<u16> for CommandCode {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Self::iter().find(|e| e.clone() as u16 == v).ok_or(())
    }
}

impl rand::distributions::Distribution<CommandCode> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> CommandCode {
        CommandCode::iter()
            .nth(rng.gen_range(0..CommandCode::iter().len()))
            .unwrap()
    }
}
