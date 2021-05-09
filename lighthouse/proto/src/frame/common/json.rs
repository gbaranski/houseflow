use crate::{DecodeError, Decoder, Encoder, Framed};
use bytes::{Buf, BufMut};
use lighthouse_macros::decoder;

impl Decoder for serde_json::Value {
    const MIN_SIZE: usize = 0;

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let json = serde_json::from_reader(buf.reader())?;
        Ok(json)
    }
}

impl Encoder for serde_json::Value {
    fn encode(&self, buf: &mut impl BufMut) {
        let bytes = serde_json::to_vec(self).expect("invalid JSON");
        buf.put_slice(&bytes);
    }
}

impl<'de> Framed<'de> for serde_json::Value {}

