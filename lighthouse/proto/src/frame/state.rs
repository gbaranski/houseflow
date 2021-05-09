use bytes::{BufMut, Buf};
use crate::{DecodeError, Decoder, Encoder};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Frame {
    pub state: serde_json::Value,
}

impl Into<crate::Frame> for Frame {
    fn into(self) -> crate::Frame {
        crate::Frame::State(self)
    }
}


impl Decoder for Frame {
    const MIN_SIZE: usize = 0;

    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        if buf.remaining() < Self::MIN_SIZE {
            return Err(DecodeError::InvalidSize {
                expected: Self::MIN_SIZE,
                received: buf.remaining(),
            });
        }
        let state: serde_json::Value = serde_json::from_reader(buf.reader())?;

        Ok(Self {
            state,
        })
    }
}

impl Encoder for Frame {
    fn encode(&self, buf: &mut impl BufMut) {
        let state_bytes = serde_json::to_vec(&self.state).expect("invalid state");
        buf.put_slice(&state_bytes);
    }
}
