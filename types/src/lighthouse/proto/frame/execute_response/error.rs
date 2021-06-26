use crate::lighthouse::proto::{DecodeError, Decoder, Encoder, Framed};
use crate::DeviceError;
use bytes::{Buf, BufMut};
use houseflow_macros::decoder;
use std::convert::TryFrom;
use std::mem::size_of;

impl Decoder for DeviceError {
    const MIN_SIZE: usize = size_of::<Self>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let v = buf.get_u16();
        Self::try_from(v).map_err(|_| DecodeError::InvalidField {
            field: std::any::type_name::<Self>(),
        })
    }
}

impl Encoder for DeviceError {
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u16(self.clone() as u16);
    }
}

impl<'de> Framed<'de> for DeviceError {}
