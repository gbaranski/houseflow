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

impl Into<command::Frame> for ActorCommandFrame {
    fn into(self) -> command::Frame {
        self.inner
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


impl Into<command_response::Frame> for ActorCommandResponseFrame {
    fn into(self) -> command_response::Frame {
        self.inner
    }
}

impl Message for ActorCommandFrame {
    type Result = Result<ActorCommandResponseFrame, DeviceError>;
}

pub struct ActorStateCheckFrame {
    inner: state_check::Frame,
}

impl Into<state_check::Frame> for ActorStateCheckFrame {
    fn into(self) -> state_check::Frame {
        self.inner
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
