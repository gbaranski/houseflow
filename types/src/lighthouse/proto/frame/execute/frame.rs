use crate::lighthouse::proto::{DecodeError, Decoder, Encoder, Frame, FrameID, Framed};
use bytes::{Buf, BufMut};
use houseflow_macros::decoder;
use serde::{Deserialize, Serialize};
use std::mem::size_of;
use crate::DeviceCommand;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecuteFrame {
    pub id: FrameID,
    pub command: DeviceCommand,
    pub params: serde_json::Value,
}

impl<'de> Framed<'de> for ExecuteFrame {}

impl Decoder for ExecuteFrame {
    const MIN_SIZE: usize = size_of::<FrameID>() + size_of::<DeviceCommand>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let id = FrameID::decode(buf)?;
        let command = DeviceCommand::decode(buf)?;
        let params = serde_json::Value::decode(buf)?;

        Ok(Self {
            id,
            command,
            params,
        })
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
