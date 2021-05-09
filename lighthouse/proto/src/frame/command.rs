use crate::{DecodeError, Decoder, Encoder};
use super::FrameID;
use bytes::{Buf, BufMut};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::mem::size_of;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
pub struct Frame {
    pub id: FrameID,
    pub command: Command,
    pub params: serde_json::Value,
}

impl Into<crate::Frame> for Frame {
    fn into(self) -> crate::Frame {
        crate::Frame::Command(self)
    }
}


impl Frame {
    pub fn new(command: Command, params: serde_json::Value) -> Self {
        Self {
            id: rand::random(),
            command,
            params,
        }
    }
}

impl Decoder for Frame {
    const MIN_SIZE: usize = size_of::<FrameID>() + size_of::<Command>();

    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < Self::MIN_SIZE {
            return Err(DecodeError::InvalidSize {
                expected: Self::MIN_SIZE,
                received: buf.remaining(),
            });
        }

        let id = buf.get_u16();
        let command = buf
            .get_u16()
            .try_into()
            .map_err(|_| DecodeError::InvalidField { field: "command" })?;
        let params: serde_json::Value = serde_json::from_reader(buf.reader())?;

        Ok(Self {
            id,
            command,
            params,
        })
    }
}

impl Encoder for Frame {
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u16(self.id);
        buf.put_u16(self.command as u16);
        let params_bytes = serde_json::to_vec(&self.params).expect("invalid params");
        buf.put_slice(&params_bytes);
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[repr(u16)]
pub enum Command {
    NoOperation = 0x0000,
    OnOff = 0x0001,
}

impl rand::distributions::Distribution<Command> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Command {
        Command::iter()
            .nth(rng.gen_range(0..Command::iter().len()))
            .unwrap()
    }
}

impl TryFrom<u16> for Command {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Self::iter().find(|e| *e as u16 == v).ok_or(())
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::NoOperation
    }
}
