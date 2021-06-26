use crate::lighthouse::proto::{DecodeError, Decoder, Encoder, Framed};
use bytes::{Buf, BufMut};
use houseflow_macros::decoder;
use std::convert::TryFrom;
use std::mem::size_of;
use crate::DeviceStatus;

impl Decoder for DeviceStatus {
    const MIN_SIZE: usize = size_of::<Self>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let v = buf.get_u8();
        Self::try_from(v).map_err(|_| DecodeError::InvalidField {
            field: std::any::type_name::<Self>(),
        })
    }
}

impl Encoder for DeviceStatus {
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.clone() as u8);
    }
}

impl<'de> Framed<'de> for DeviceStatus {}
