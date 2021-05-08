use actix::prelude::{Message, MessageResponse};
use lighthouse_proto::{execute, execute_response, Frame, FrameID};
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Device is not connected")]
    DeviceNotConnected,

    #[error("Timeout when sending request")]
    Timeout,
}

#[derive(Debug, Clone)]
pub enum Request {
    Execute(execute::Frame),
}


#[derive(Debug, Clone, MessageResponse)]
pub enum Response {
    Execute(execute_response::Frame),
}

impl Message for Request {
    type Result = std::result::Result<Response, RequestError>;
}

impl TryFrom<Frame> for Response {
    type Error = ();

    fn try_from(frame: Frame) -> Result<Self, Self::Error> {
        match frame {
            Frame::ExecuteResponse(frame) => Ok(Response::Execute(frame)),
            _ => Err(()),
        }
    }
}

impl Into<Frame> for Request {
    fn into(self) -> Frame {
        match self {
            Self::Execute(frame) => Frame::Execute(frame),
        }
    }
}

impl Into<Frame> for Response {
    fn into(self) -> Frame {
        match self {
            Self::Execute(frame) => Frame::ExecuteResponse(frame),
        }
    }
}

impl Request {
    pub fn id(&self) -> FrameID {
        match self {
            Self::Execute(frame) => frame.id,
        }
    }
}

impl Response {
    pub fn id(&self) -> FrameID {
        match self {
            Self::Execute(frame) => frame.id,
        }
    }
}
