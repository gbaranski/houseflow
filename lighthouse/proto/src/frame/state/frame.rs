use crate::{DecodeError, Decoder, Encoder, Frame};
use bytes::{Buf, BufMut};
use lighthouse_macros::decoder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StateFrame {
    pub state: serde_json::Value,
}

impl Decoder for StateFrame {
    const MIN_SIZE: usize = 0;

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let state = serde_json::Value::decode(buf)?;

        Ok(Self { state })
    }
}

impl Encoder for StateFrame {
    fn encode(&self, buf: &mut impl BufMut) {
        self.state.encode(buf);
    }
}

impl From<StateFrame> for Frame {
    fn from(val: StateFrame) -> Self {
        Frame::State(val)
    }
}
