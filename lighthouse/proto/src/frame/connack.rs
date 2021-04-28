use std::convert::TryFrom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Frame {
    pub response_code: ResponseCode,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ResponseCode {
    Accepted = 0x00,
    Unauthorized = 0x01,
}

impl Default for ResponseCode {
    fn default() -> Self {
        Self::Accepted
    }
}

impl TryFrom<u8> for ResponseCode {
    type Error = ();

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        Self::iter().find(|v| *v as u8 == item).ok_or(())
    }
}

impl rand::distributions::Distribution<ResponseCode> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ResponseCode {
        ResponseCode::iter()
            .nth(rng.gen_range(0..ResponseCode::iter().len()))
            .unwrap()
    }
}
