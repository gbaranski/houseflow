pub mod execute;
pub mod execute_response;

pub mod query;
pub mod state;

pub type FrameID = u16;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "type", rename_all = "kebab-case")]
#[repr(u8)]
pub enum Frame {
    /// Packet which will received from device to share its state
    State(state::Frame),

    /// Packet which will be send to get current state from device
    Query(query::Frame),

    /// Packet which will be send to execute some action on client side
    Execute(execute::Frame),

    /// Packet which will be send as a response to Execute request from server
    ExecuteResponse(execute_response::Frame),
}

impl Frame {
    pub fn name(&self) -> &'static str {
        match self {
            Self::State(_) => "state",
            Self::Query(_) => "query",
            Self::Execute(_) => "execute",
            Self::ExecuteResponse(_) => "execute-response",
        }
    }
}
