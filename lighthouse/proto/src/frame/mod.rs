pub mod execute;
pub mod execute_response;

use std::convert::TryFrom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum Frame {
    /// Placeholder, MUST NOT be used
    ///
    /// Opcode: 0x00
    NoOperation,

    /// Packet which will be send to execute some action on client side
    ///
    /// Opcode: 0x03
    Execute(execute::Frame),

    /// Packet which will be send as a response to Execute request from server
    ///
    /// Opcode: 0x04
    ExecuteResponse(execute_response::Frame),
}

impl Default for Frame {
    fn default() -> Self {
        Self::NoOperation
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
#[repr(u8)]
pub enum Opcode {
    /// Placeholder, MUST NOT be used
    ///
    /// Opcode: 0x00
    NoOperation,

    /// Packet which will be send to execute some action on client side
    ///
    /// Opcode: 0x01
    Execute,

    /// Packet which will be send as a response to Execute request from server
    ///
    /// Opcode: 0x02
    ExecuteResponse,
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Opcode::iter().find(|e| *e as u8 == v).ok_or(())
    }
}

impl Frame {
    pub fn opcode(&self) -> Opcode {
        // sorry for that, but discriminants on non-unit variants are experimental
        match self {
            Frame::NoOperation => Opcode::NoOperation,
            Frame::Execute(_) => Opcode::Execute,
            Frame::ExecuteResponse(_) => Opcode::ExecuteResponse,
        }
    }
}
