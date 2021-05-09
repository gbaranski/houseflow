use crate::{DecodeError, Decoder, Encoder};
use super::FrameID;
use bytes::{Buf, BufMut};
use std::convert::{TryFrom, TryInto};
use std::mem::size_of;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Frame {
    pub id: FrameID,
    pub response_code: ResponseCode,
    pub error: Error,
    pub state: serde_json::Value,
}

impl Into<crate::Frame> for Frame {
    fn into(self) -> crate::Frame {
        crate::Frame::CommandResponse(self)
    }
}


impl Decoder for Frame {
    const MIN_SIZE: usize = size_of::<FrameID>() + size_of::<ResponseCode>() + size_of::<Error>();

    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < Self::MIN_SIZE {
            return Err(DecodeError::InvalidSize {
                expected: Self::MIN_SIZE,
                received: buf.remaining(),
            });
        }

        let id = buf.get_u16();
        let response_code = buf
            .get_u8()
            .try_into()
            .map_err(|_| DecodeError::InvalidField {
                field: "response_code",
            })?;
        let error = buf
            .get_u16()
            .try_into()
            .map_err(|_| DecodeError::InvalidField { field: "error" })?;
        let state: serde_json::Value = serde_json::from_reader(buf.reader())?;

        Ok(Self {
            id,
            response_code,
            error,
            state,
        })
    }
}

impl Encoder for Frame {
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u16(self.id);
        buf.put_u8(self.response_code as u8);
        buf.put_u16(self.error as u16);
        let state_bytes = serde_json::to_vec(&self.state).expect("invalid state");
        buf.put_slice(&state_bytes);
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ResponseCode {
    /// Confirm that the command succeeded.
    Success,

    /// Command is enqueued but expected to succeed.
    Pending,

    /// There is an issue or alert associated with a command. The command could succeed or fail.
    /// This status type is typically set when you want to send additional information about another connected device.
    Exceptions,

    /// Target device is unable to perform the command.
    Error,
}

impl rand::distributions::Distribution<ResponseCode> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ResponseCode {
        ResponseCode::iter()
            .nth(rng.gen_range(0..ResponseCode::iter().len()))
            .unwrap()
    }
}

impl TryFrom<u8> for ResponseCode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, ()> {
        Self::iter().find(|e| *e as u8 == v).ok_or(())
    }
}

impl Default for ResponseCode {
    fn default() -> Self {
        Self::Success
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum Error {
    /// No error occurred
    None = 0x0000,

    /// Actually, <device(s)> <doesn't/don't> support that functionality.
    FunctionNotSupported = 0x0001,
}

impl rand::distributions::Distribution<Error> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Error {
        Error::iter()
            .nth(rng.gen_range(0..Error::iter().len()))
            .unwrap()
    }
}

impl TryFrom<u16> for Error {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Self::iter().find(|e| *e as u16 == v).ok_or(())
    }
}

impl Default for Error {
    fn default() -> Self {
        Self::None
    }
}
