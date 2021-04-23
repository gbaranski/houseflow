use strum_macros::EnumIter;

mod codec;
mod frame;

pub const CLIENT_ID_SIZE: usize = 16;
pub type ClientID = [u8; CLIENT_ID_SIZE];

#[derive(Debug)]
pub enum Error {
    InvalidSize { expected: usize, received: usize },
    InvalidOpcode(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, EnumIter)]
#[repr(u8)]
pub enum Opcode {
    Connect = 0x01,
    ConnACK = 0x02,
}
