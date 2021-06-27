use actix::{Message, MessageResponse};
use houseflow_types::lighthouse::{
    proto::{execute, execute_response, state, query},
    DeviceCommunicationError,
};

pub struct ActorExecuteFrame {
    inner: execute::Frame,
}

impl From<execute::Frame> for ActorExecuteFrame {
    fn from(v: execute::Frame) -> Self {
        Self { inner: v }
    }
}

impl From<ActorExecuteFrame> for execute::Frame {
    fn from(val: ActorExecuteFrame) -> Self {
        val.inner
    }
}

#[derive(MessageResponse)]
pub struct ActorExecuteResponseFrame {
    inner: execute_response::Frame,
}

impl From<execute_response::Frame> for ActorExecuteResponseFrame {
    fn from(v: execute_response::Frame) -> Self {
        Self { inner: v }
    }
}

impl From<ActorExecuteResponseFrame> for execute_response::Frame {
    fn from(val: ActorExecuteResponseFrame) -> Self {
        val.inner
    }
}

impl Message for ActorExecuteFrame {
    type Result = Result<ActorExecuteResponseFrame, DeviceCommunicationError>;
}

pub struct ActorQueryFrame {
    inner: query::Frame,
}

impl From<ActorQueryFrame> for query::Frame {
    fn from(val: ActorQueryFrame) -> Self {
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

impl Message for ActorQueryFrame {
    type Result = Result<ActorStateFrame, DeviceCommunicationError>;
}
