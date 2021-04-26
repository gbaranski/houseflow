use crate::{ClientID, Opcode};
use std::convert::TryFrom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ResponseCode {
    ConnectionAccepted = 0x00,
    Unauthorized = 0x01,
}

impl Default for ResponseCode {
    fn default() -> Self {
        Self::ConnectionAccepted
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

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Frame {
    /// First frame that should be sent from Client to Server
    ///
    /// Opcode: 0x01
    Connect { client_id: ClientID },

    /// First frame that should be sent from Server to Client as a response for Connect
    ///
    /// Opcode: 0x02
    ConnACK { response_code: ResponseCode },
}

impl Frame {
    pub fn opcode(&self) -> Opcode {
        match self {
            Self::Connect { .. } => Opcode::Connect,
            Self::ConnACK { .. } => Opcode::ConnACK,
        }
    }
}
