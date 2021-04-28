use std::convert::TryFrom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Frame {
    pub id: u32,
    pub response_code: ResponseCode,
    pub error: Error,
    pub state: serde_json::Value,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ResponseCode {
    /// Confirm that the command succeeded.
    Success = 0x01,

    /// Command is enqueued but expected to succeed.
    Pending = 0x02,

    /// Target device is in offline state or unreachable.
    // Yes, it does not make a lot of sense to put that here, but I'd like to satisfy Google API as
    // defned here https://developers.google.com/assistant/smarthome/reference/intent/execute
    Offline = 0x03,

    /// There is an issue or alert associated with a command. The command could succeed or fail.
    /// This status type is typically set when you want to send additional information about another connected device.
    Exceptions = 0x04,

    /// Target device is unable to perform the command.
    Error = 0x05,
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
