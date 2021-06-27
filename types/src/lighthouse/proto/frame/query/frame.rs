use crate::lighthouse::proto::{DecodeError, Decoder, Encoder, Frame, Framed};
use bytes::{Buf, BufMut};
use houseflow_macros::decoder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QueryFrame {}

impl Decoder for QueryFrame {
    const MIN_SIZE: usize = 0;

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        Ok(Self {})
    }
}

impl Encoder for QueryFrame {
    fn encode(&self, _buf: &mut impl BufMut) {}
}

impl<'de> Framed<'de> for QueryFrame {}

impl From<QueryFrame> for Frame {
    fn from(val: QueryFrame) -> Self {
        Frame::Query(val)
    }
}
