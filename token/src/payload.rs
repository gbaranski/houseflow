use std::convert::TryInto;
use crate::SizedFrame;

pub const AUDIENCE_SIZE: usize = 36;

pub struct Payload {
    pub audience: [u8; AUDIENCE_SIZE],
    pub expires_at: u32,
}

impl SizedFrame for Payload {
    // 4 first bytes are unsigned 32 bit integer
    const SIZE: usize = 4 + AUDIENCE_SIZE;
}

impl Payload {
    pub fn from_bytes(b: [u8; Self::SIZE]) -> Self {
        Self {
            expires_at: u32::from_be_bytes(b[0 .. 4].try_into().unwrap()),
            audience: b[4 .. 4+AUDIENCE_SIZE]
                .try_into()
                .unwrap(),
        }
    }

    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        let expires_at_bytes = self.expires_at.to_be_bytes();
        let vector = [&expires_at_bytes[..], &self.audience[..]].concat();

        vector.try_into().unwrap()
    }

}

