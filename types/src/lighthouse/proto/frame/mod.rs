pub mod execute;
pub mod execute_response;

pub mod query;
pub mod state;

pub mod no_operation;

mod common;
mod opcode;

pub use common::FrameID;
pub(crate) use opcode::Opcode;

use super::{Decoder, Encoder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Frame {
    /// No operation, nothing should happen
    ///
    /// Opcode: 0x00
    NoOperation(no_operation::Frame),

    /// Packet which will received from device to share its state
    ///
    /// Opcode: 0x01
    State(state::Frame),

    /// Packet which will be send to get current state from device
    ///
    /// Opcode: 0x02
    Query(query::Frame),

    /// Packet which will be send to execute some action on client side
    ///
    /// Opcode: 0x03
    Execute(execute::Frame),

    /// Packet which will be send as a response to Execute request from server
    ///
    /// Opcode: 0x04
    ExecuteResponse(execute_response::Frame),
}

pub trait Framed<'de>:
    std::fmt::Debug + Clone + Eq + PartialEq + Serialize + Deserialize<'de> + Encoder + Decoder
{
}

impl Frame {
    pub(crate) const fn opcode(&self) -> Opcode {
        // sorry for that, but discriminants on non-unit variants are experimental
        match self {
            Frame::NoOperation(_) => Opcode::NoOperation,
            Frame::State(_) => Opcode::State,
            Frame::Query(_) => Opcode::Query,
            Frame::Execute(_) => Opcode::Execute,
            Frame::ExecuteResponse(_) => Opcode::ExecuteResponse,
        }
    }
}
