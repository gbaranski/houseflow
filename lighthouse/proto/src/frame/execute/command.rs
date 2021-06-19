use crate::{DecodeError, Decoder, Encoder, Framed};
use bytes::{Buf, BufMut};
use lighthouse_macros::decoder;
use std::{convert::TryInto, mem::size_of};
use types::DeviceCommand;

impl<'de> Framed<'de> for DeviceCommand {}

impl Encoder for DeviceCommand {
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u16(self.clone() as u16);
    }
}

impl Decoder for DeviceCommand {
    const MIN_SIZE: usize = size_of::<u16>();

    #[decoder]
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        buf.get_u16()
            .try_into()
            .map_err(|_| DecodeError::InvalidField {
                field: std::any::type_name::<Self>(),
            })
    }
}
