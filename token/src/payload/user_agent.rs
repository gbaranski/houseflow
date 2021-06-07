use crate::{DecodeError, Decoder, Encoder};
use types::UserAgent;
use std::convert::TryFrom;

impl Decoder for UserAgent {
    const SIZE: usize = std::mem::size_of::<UserAgent>();

    fn decode(buf: &mut impl bytes::Buf) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        if buf.remaining() < Self::SIZE {
            return Err(DecodeError::InvalidLength {
                expected: Self::SIZE,
                received: buf.remaining(),
            });
        }
        let byte = buf.get_u8();
        Self::try_from(byte).map_err(|_| DecodeError::UnknownUserAgent(byte))
    }
}

impl Encoder for UserAgent {
    fn encode(&self, buf: &mut impl bytes::BufMut) {
        buf.put_u8(*self as u8)
    }
}

