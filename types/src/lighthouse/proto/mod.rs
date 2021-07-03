pub mod execute;
pub mod execute_response;

pub mod query;
pub mod state;

pub type FrameID = u16;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[repr(u8)]
pub enum Frame {
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
