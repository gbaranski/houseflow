use std::convert::TryFrom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
#[repr(u8)]
pub(crate) enum Opcode {
    NoOperation,
    State,
    StateCheck,
    Command,
    CommandResponse,
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Self::iter().find(|e| *e as u8 == v).ok_or(())
    }
}
