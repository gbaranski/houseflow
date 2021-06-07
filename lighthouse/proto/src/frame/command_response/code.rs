use crate::{DecodeError, Decoder, Encoder, Framed};
use bytes::{Buf, BufMut};
use macros::decoder;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::mem::size_of;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
#[repr(u8)]
pub enum CommandResponseCode {
    /// Confirm that the command succeeded.
    Success,

    /// Command is enqueued but expected to succeed.
    Pending,

    /// There is an issue or alert associated with a command. The command could succeed or fail.
    Exceptions,

    /// Target device is unable to perform the command.
    Error,
}

impl Decoder for CommandResponseCode {
    const MIN_SIZE: usize = size_of::<Self>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let v = buf.get_u8();
        Self::try_from(v).map_err(|_| DecodeError::InvalidField {
            field: "response_code",
        })
    }
}

impl Encoder for CommandResponseCode {
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.clone() as u8);
    }
}

impl<'de> Framed<'de> for CommandResponseCode {}

impl TryFrom<u8> for CommandResponseCode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, ()> {
        Self::iter().find(|e| e.clone() as u8 == v).ok_or(())
    }
}

impl rand::distributions::Distribution<CommandResponseCode> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> CommandResponseCode {
        CommandResponseCode::iter()
            .nth(rng.gen_range(0..CommandResponseCode::iter().len()))
            .unwrap()
    }
}
