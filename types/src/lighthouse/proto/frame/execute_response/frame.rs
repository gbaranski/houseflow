use crate::lighthouse::proto::{DecodeError, Decoder, Encoder, Frame, FrameID, Framed};
use crate::{DeviceError, DeviceStatus};
use bytes::{Buf, BufMut};
use houseflow_macros::decoder;
use serde::{Deserialize, Serialize};
use std::mem::size_of;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecuteResponseFrame {
    pub id: FrameID,
    pub status: DeviceStatus,
    pub error: DeviceError,
    pub state: serde_json::Value,
}

impl<'de> Framed<'de> for ExecuteResponseFrame {}

impl Decoder for ExecuteResponseFrame {
    const MIN_SIZE: usize =
        size_of::<FrameID>() + size_of::<DeviceStatus>() + size_of::<DeviceError>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let id = FrameID::decode(buf)?;
        let status = DeviceStatus::decode(buf)?;
        let error = DeviceError::decode(buf)?;
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
