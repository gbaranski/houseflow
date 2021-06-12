use actix::{Message, MessageResponse};
use lighthouse_proto::{command, command_response, state, state_check};
use lighthouse_types::DeviceError;

pub struct ActorCommandFrame {
    inner: command::Frame,
}

impl From<command::Frame> for ActorCommandFrame {
    fn from(v: command::Frame) -> Self {
        Self { inner: v }
    }
}

impl From<ActorCommandFrame> for command::Frame {
    fn from(val: ActorCommandFrame) -> Self {
        val.inner
    }
}

#[derive(MessageResponse)]
pub struct ActorCommandResponseFrame {
    inner: command_response::Frame,
}


impl From<command_response::Frame> for ActorCommandResponseFrame {
    fn from(v: command_response::Frame) -> Self {
        Self { inner: v }
    }
}


impl From<ActorCommandResponseFrame> for command_response::Frame {
    fn from(val: ActorCommandResponseFrame) -> Self {
        val.inner
    }
}

impl Message for ActorCommandFrame {
    type Result = Result<ActorCommandResponseFrame, DeviceError>;
}

pub struct ActorStateCheckFrame {
    inner: state_check::Frame,
}

impl From<ActorStateCheckFrame> for state_check::Frame {
    fn from(val: ActorStateCheckFrame) -> Self {
        val.inner
    }
}

#[derive(MessageResponse)]
pub struct ActorStateFrame {
    #[allow(dead_code)]
    inner: state::Frame,
}


impl From<state::Frame> for ActorStateFrame {
    fn from(v: state::Frame) -> Self {
        Self { inner: v }
    }
}
// impl Into<state::Frame> for ActorStateFrame {
//     fn into(self) -> state::Frame {
//         self.inner
//     }
// }

impl Message for ActorStateCheckFrame {
    type Result = Result<ActorStateFrame, DeviceError>;
}
