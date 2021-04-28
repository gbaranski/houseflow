use std::convert::TryFrom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Frame {
    pub id: u32,
    pub command: Command,
    pub params: serde_json::Value,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum Command {
    NoOperation = 0x0000,
    OnOff = 0x0001,
}

impl rand::distributions::Distribution<Command> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Command {
        Command::iter()
            .nth(rng.gen_range(0..Command::iter().len()))
            .unwrap()
    }
}

impl TryFrom<u16> for Command {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Self::iter().find(|e| *e as u16 == v).ok_or(())
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::NoOperation
    }
}
