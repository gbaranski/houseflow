use crate::{ClientID, Opcode};
use std::{collections::HashMap, convert::TryFrom};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ConnectionResponseCode {
    ConnectionAccepted = 0x00,
    Unauthorized = 0x01,
}

impl Default for ConnectionResponseCode {
    fn default() -> Self {
        Self::ConnectionAccepted
    }
}

impl TryFrom<u8> for ConnectionResponseCode {
    type Error = ();

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        Self::iter().find(|v| *v as u8 == item).ok_or(())
    }
}

impl rand::distributions::Distribution<ConnectionResponseCode> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ConnectionResponseCode {
        ConnectionResponseCode::iter()
            .nth(rng.gen_range(0..ConnectionResponseCode::iter().len()))
            .unwrap()
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum ExecuteCommand {
    NoOperation = 0x0000,
    OnOff = 0x0001,
}

impl rand::distributions::Distribution<ExecuteCommand> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ExecuteCommand {
        ExecuteCommand::iter()
            .nth(rng.gen_range(0..ExecuteCommand::iter().len()))
            .unwrap()
    }
}

impl TryFrom<u16> for ExecuteCommand {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Self::iter().find(|e| *e as u16 == v).ok_or(())
    }
}

impl Default for ExecuteCommand {
    fn default() -> Self {
        Self::NoOperation
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ExecuteResponseCode {
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


impl rand::distributions::Distribution<ExecuteResponseCode> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ExecuteResponseCode {
        ExecuteResponseCode::iter()
            .nth(rng.gen_range(0..ExecuteResponseCode::iter().len()))
            .unwrap()
    }
}

impl TryFrom<u8> for ExecuteResponseCode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, ()> {
        Self::iter().find(|e| *e as u8 == v).ok_or(())
    }
}

impl Default for ExecuteResponseCode {
    fn default() -> Self {
        Self::Success
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum ExecuteResponseError {
    /// No error occurred
    None = 0x0000,

    /// Actually, <device(s)> <doesn't/don't> support that functionality.
    FunctionNotSupported = 0x0001,
}

impl rand::distributions::Distribution<ExecuteResponseError> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ExecuteResponseError {
        ExecuteResponseError::iter()
            .nth(rng.gen_range(0..ExecuteResponseError::iter().len()))
            .unwrap()
    }
}

impl TryFrom<u16> for ExecuteResponseError {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Self::iter().find(|e| *e as u16 == v).ok_or(())
    }
}

impl Default for ExecuteResponseError {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum Frame {
    /// Placeholder, MUST NOT be used
    ///
    /// Opcode: 0x00
    NoOperation,

    /// First frame that should be sent from Client to Server
    ///
    /// Opcode: 0x01
    Connect { client_id: ClientID },

    /// First frame that should be sent from Server to Client as a response for Connect
    ///
    /// Opcode: 0x02
    ConnACK {
        response_code: ConnectionResponseCode,
    },

    /// Packet which will be send to execute some action on client side
    ///
    /// Opcode: 0x03
    Execute {
        id: u32,
        command: ExecuteCommand,
        params: serde_json::Value,
    },

    /// Packet which will be send as a response to Execute request from server
    ///
    /// Opcode: 0x04
    ExecuteResponse {
        id: u32,
        response_code: ExecuteResponseCode,
        error: ExecuteResponseError,
        state: serde_json::Value,
    },
}

impl Default for Frame {
    fn default() -> Self {
        Self::NoOperation
    }
}

impl Frame {
    pub fn opcode(&self) -> Opcode {
        match self {
            Self::NoOperation { .. } => Opcode::NoOperation,
            Self::Connect { .. } => Opcode::Connect,
            Self::ConnACK { .. } => Opcode::ConnACK,
            Self::Execute { .. } => Opcode::Execute,
            Self::ExecuteResponse { .. } => Opcode::ExecuteResponse,
        }
    }
}
