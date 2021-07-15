use actix::{Message, MessageResponse};
use houseflow_types::lighthouse::{
    proto::{execute, execute_response, query, state},
    DeviceCommunicationError,
};

pub struct ActorExecuteFrame {
    pub inner: execute::Frame,
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
    pub inner: execute_response::Frame,
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
    pub inner: query::Frame,
}

impl From<query::Frame> for ActorQueryFrame {
    fn from(v: query::Frame) -> Self {
        Self { inner: v }
    }
}

impl From<ActorQueryFrame> for query::Frame {
    fn from(val: ActorQueryFrame) -> Self {
        val.inner
    }
}

#[derive(MessageResponse)]
pub struct ActorStateFrame {
    #[allow(dead_code)]
    pub inner: state::Frame,
}

impl From<state::Frame> for ActorStateFrame {
    fn from(v: state::Frame) -> Self {
        Self { inner: v }
    }
}

impl From<ActorStateFrame> for state::Frame {
    fn from(val: ActorStateFrame) -> Self {
        val.inner
    }
}

impl Message for ActorQueryFrame {
    type Result = Result<ActorStateFrame, DeviceCommunicationError>;
}
