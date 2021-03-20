use std::convert::TryInto;
use crate::{Signature, Payload, SizedFrame};

pub struct Token {
    pub payload: Payload,
    pub signature: Signature,
}

impl Token {
    pub fn from_bytes(b: [u8; Self::SIZE]) -> Self {
        Self {
            signature: b[0 .. Signature::SIZE].try_into().unwrap(),
            payload: Payload::from_bytes(b[Signature::SIZE .. Self::SIZE].try_into().unwrap())
        }
    }

    pub fn to_bytes(self) -> [u8; Self::SIZE] {
        let payload_bytes = self.payload.to_bytes();
        let vector = [&self.signature[..], &payload_bytes[..]].concat();

        vector.try_into().unwrap()
    }
}

impl SizedFrame for Token {
    // 4 first bytes are unsigned 32 bit integer
    const SIZE: usize = Signature::SIZE + Payload::SIZE;
}
