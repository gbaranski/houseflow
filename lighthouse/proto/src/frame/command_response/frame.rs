use super::{Code, Error};
use crate::{DecodeError, Decoder, Encoder, FrameID, Framed, Frame};
use bytes::{Buf, BufMut};
use lighthouse_macros::decoder;
use serde::{Deserialize, Serialize};
use std::mem::size_of;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandResponseFrame {
    pub id: FrameID,
    pub code: Code,
    pub error: Error,
    pub state: serde_json::Value,
}

impl<'de> Framed<'de> for CommandResponseFrame {}

impl Decoder for CommandResponseFrame {
    const MIN_SIZE: usize = size_of::<FrameID>() + size_of::<Code>() + size_of::<Error>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let id = FrameID::decode(buf)?;
        let code = Code::decode(buf)?;
        let error = Error::decode(buf)?;
        let state = serde_json::Value::decode(buf)?;

        Ok(Self {
            id,
            code,
            error,
            state,
        })
    }
}

impl Encoder for CommandResponseFrame {
    fn encode(&self, buf: &mut impl BufMut) {
        self.id.encode(buf);
        self.code.encode(buf);
        self.error.encode(buf);
        self.state.encode(buf);
    }
}

impl Into<Frame> for CommandResponseFrame {
    fn into(self) -> Frame {
        Frame::CommandResponse(self)
    }
}
