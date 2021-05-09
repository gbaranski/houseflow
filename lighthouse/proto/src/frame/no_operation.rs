use crate::{DecodeError, Decoder, Encoder};
use bytes::{Buf, BufMut};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
pub struct Frame {}

impl Into<crate::Frame> for Frame {
    fn into(self) -> crate::Frame {
        crate::Frame::NoOperation(self)
    }
}

impl Frame {
    pub fn new() -> Self {
        Self {}
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

        Ok(Self {})
    }
}

impl Encoder for Frame {
    fn encode(&self, _buf: &mut impl BufMut) {}
}
