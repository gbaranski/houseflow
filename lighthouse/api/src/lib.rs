use lighthouse_proto::{Frame, frame};
use thiserror::Error;
use actix::prelude::Message;

#[derive(Debug, Clone)]
pub enum Request {
    Execute(frame::execute::Frame),
}


#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Device is not connected")]
    DeviceNotConnected,

    #[error("Timeout when sending request")]
    Timeout,
}

#[derive(Debug, Clone)]
pub enum Response {
    Execute(frame::execute_response::Frame)
}

impl Message for Request {
    type Result = ();
}

impl Message for Response {
    type Result = ();
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


