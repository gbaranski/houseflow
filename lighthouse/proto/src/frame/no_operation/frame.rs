use crate::{DecodeError, Decoder, Encoder, Framed, Frame};
use lighthouse_macros::decoder;
use bytes::{Buf, BufMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NoOperationFrame {}

impl Decoder for NoOperationFrame {
    const MIN_SIZE: usize = 0;

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

impl Encoder for NoOperationFrame {
    fn encode(&self, _buf: &mut impl BufMut) {}
}

impl<'de> Framed<'de> for NoOperationFrame {}

impl Into<Frame> for NoOperationFrame {
    fn into(self) -> Frame {
        Frame::NoOperation(self)
    }
}
