use std::convert::TryFrom;
use strum::{IntoEnumIterator, EnumIter};

#[derive(Debug, Clone, Copy, EnumIter)]
#[repr(u8)]
pub(crate) enum Opcode {
    NoOperation,
    State,
    StateCheck,
    Execute,
    ExecuteResponse,
}

impl From<Opcode> for u8 {
    fn from(val: Opcode) -> Self {
        val as u8
    }
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        Self::iter().find(|e| *e as u8 == v).ok_or(())
    }
}
