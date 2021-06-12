use crate::{DecodeError, Decoder, Encoder, Framed, Frame};
use bytes::{Buf, BufMut};
use lighthouse_macros::decoder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StateCheckFrame {}

impl Decoder for StateCheckFrame {
    const MIN_SIZE: usize = 0;

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

impl Encoder for StateCheckFrame {
    fn encode(&self, _buf: &mut impl BufMut) {}
}

impl<'de> Framed<'de> for StateCheckFrame {}

impl From<StateCheckFrame> for Frame {
    fn from(val: StateCheckFrame) -> Self {
        Frame::StateCheck(val)
    }
}
