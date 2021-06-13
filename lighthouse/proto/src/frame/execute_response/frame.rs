use super::{Status, Error};
use crate::{DecodeError, Decoder, Encoder, Frame, FrameID, Framed};
use bytes::{Buf, BufMut};
use lighthouse_macros::decoder;
use serde::{Deserialize, Serialize};
use std::mem::size_of;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecuteResponseFrame {
    pub id: FrameID,
    pub status: Status,
    pub error: Error,
    pub state: serde_json::Value,
}

impl<'de> Framed<'de> for ExecuteResponseFrame {}

impl Decoder for ExecuteResponseFrame {
    const MIN_SIZE: usize = size_of::<FrameID>() + size_of::<Status>() + size_of::<Error>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let id = FrameID::decode(buf)?;
        let status = Status::decode(buf)?;
        let error = Error::decode(buf)?;
        let state = serde_json::Value::decode(buf)?;

        Ok(Self {
            id,
            status,
            error,
            state,
        })
    }
}

impl Encoder for ExecuteResponseFrame {
    fn encode(&self, buf: &mut impl BufMut) {
        self.id.encode(buf);
        self.status.encode(buf);
        self.error.encode(buf);
        self.state.encode(buf);
    }
}

impl From<ExecuteResponseFrame> for Frame {
    fn from(val: ExecuteResponseFrame) -> Self {
        Frame::ExecuteResponse(val)
    }
}
