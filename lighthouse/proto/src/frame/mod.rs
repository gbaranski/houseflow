pub mod command;
pub mod command_response;

pub mod state;
pub mod state_check;

pub mod no_operation;

use std::convert::TryFrom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type FrameID = u16;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum Frame {
    /// Placeholder, MUST NOT be used
    ///
    /// Opcode: 0x00
    NoOperation(no_operation::Frame),

    /// Packet which will be send to get current state from device
    ///
    /// Opcode: 0x03
    State(state::Frame),

    /// Packet which will be send to get current state from device
    ///
    /// Opcode: 0x03
    StateCheck(state_check::Frame),

    /// Packet which will be send to execute some action on client side
    ///
    /// Opcode: 0x01
    Command(command::Frame),

    /// Packet which will be send as a response to Execute request from server
    ///
    /// Opcode: 0x02
    CommandResponse(command_response::Frame),
}


impl Default for Frame {
    fn default() -> Self {
        Self::NoOperation(no_operation::Frame::new())
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
#[repr(u8)]
pub(crate) enum Opcode {
    NoOperation,
    State,
    StateCheck,
    Command,
    CommandResponse,
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
    pub(crate) fn opcode(&self) -> Opcode {
        // sorry for that, but discriminants on non-unit variants are experimental
        match self {
            Frame::NoOperation(_) => Opcode::NoOperation,
            Frame::State(_) => Opcode::State,
            Frame::StateCheck(_) => Opcode::StateCheck,
            Frame::Command(_) => Opcode::Command,
            Frame::CommandResponse(_) => Opcode::CommandResponse,
        }
    }
}

