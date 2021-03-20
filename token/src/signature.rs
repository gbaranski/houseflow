use crate::SizedFrame;

const SIGNATURE_SIZE: usize = 32; // SHA256 bytes

pub type Signature = [u8; SIGNATURE_SIZE];

impl SizedFrame for Signature {
    const SIZE: usize = SIGNATURE_SIZE;
}
