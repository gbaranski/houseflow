pub mod command;
pub mod command_response;

pub mod state;
pub mod state_check;

pub mod no_operation;

mod common;
mod opcode;

pub use common::FrameID;
pub(crate) use opcode::Opcode;

use crate::{Decoder, Encoder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

pub trait Framed<'de>:
    std::fmt::Debug + Clone + Eq + PartialEq + Serialize + Deserialize<'de> + Encoder + Decoder
{
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
