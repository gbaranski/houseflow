use lighthouse_proto::frame::{Frame, self};
mod store;
mod error;
mod channels;
mod io;

pub use store::Store;
pub use error::{Error, RequestError};
pub use io::run;


#[derive(Debug, Clone)]
pub enum Request {
    Execute(frame::execute::Frame)
}

#[derive(Debug, Clone)]
pub enum Response {
    Execute(frame::execute_response::Frame)
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


