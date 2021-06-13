use super::Command;
use crate::{DecodeError, Decoder, Encoder};
use crate::{Frame, FrameID, Framed};
use bytes::{Buf, BufMut};
use lighthouse_macros::decoder;
use serde::{Deserialize, Serialize};
use std::mem::size_of;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecuteFrame {
    pub id: FrameID,
    pub command: Command,
    pub params: serde_json::Value,
}

impl<'de> Framed<'de> for ExecuteFrame {}

impl Decoder for ExecuteFrame {
    const MIN_SIZE: usize = size_of::<FrameID>() + size_of::<Command>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let id = FrameID::decode(buf)?;
        let command = Command::decode(buf)?;
        let params = serde_json::Value::decode(buf)?;

        Ok(Self { id, command, params })
    }
}

impl Encoder for ExecuteFrame {
    fn encode(&self, buf: &mut impl BufMut) {
        self.id.encode(buf);
        self.command.encode(buf);
        self.params.encode(buf);
    }
}

impl From<ExecuteFrame> for Frame {
    fn from(val: ExecuteFrame) -> Self {
        Frame::Execute(val)
    }
}
