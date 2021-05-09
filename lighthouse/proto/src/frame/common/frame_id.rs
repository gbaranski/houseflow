use crate::{DecodeError, Decoder, Encoder, Framed};
use bytes::{Buf, BufMut};
use lighthouse_macros::decoder;
use serde::{Deserialize, Serialize};
use std::mem::size_of;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct FrameID {
    inner: u16,
}

impl Encoder for FrameID {
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u16(self.inner);
    }
}

impl Decoder for FrameID {
    const MIN_SIZE: usize = size_of::<u16>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let inner = buf.get_u16();
        Ok(Self { inner })
    }
}

impl<'de> Framed<'de> for FrameID {}


impl rand::distributions::Distribution<FrameID> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> FrameID {
        FrameID { inner: rng.gen() }
    }
}
