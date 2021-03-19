#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::convert::TryInto;

pub enum TokenFrame {
    Signature,
    ExpiresAt,
    Audience,
}

const AUDIENCE_SIZE: usize = 36;  // UUID
const SIGNATURE_SIZE: usize = 32; // SHA256
const EXPIRES_AT_SIZE: usize = 4; // u32

const TOKEN_SIZE: usize = 
    SIGNATURE_SIZE + 
    EXPIRES_AT_SIZE + 
    AUDIENCE_SIZE;


pub enum Error {
    InvalidFrameSize(TokenFrame)
}

pub struct Token<'a> {
    pub signature: &'a [u8; SIGNATURE_SIZE],
    pub audience: &'a [u8; AUDIENCE_SIZE],
    pub expires_at: u32,
}


impl<'a> Token<'a> {
    pub fn from_bytes(bytes: &[u8; TOKEN_SIZE]) -> Result<Token, Error> {
        type ByteOffset = (usize, usize);
        const SIGNATURE_OFFSET: ByteOffset  = (0                   , 0                   + SIGNATURE_SIZE);
        const EXPIRES_AT_OFFSET: ByteOffset = (SIGNATURE_OFFSET.1  , SIGNATURE_OFFSET.1  + EXPIRES_AT_SIZE);
        const AUDIENCE_OFFSET: ByteOffset   = (EXPIRES_AT_OFFSET.1 , EXPIRES_AT_OFFSET.1 + AUDIENCE_SIZE);

        let signature: &[u8; SIGNATURE_SIZE] = bytes[SIGNATURE_OFFSET.0 .. SIGNATURE_OFFSET.1].try_into()
            .map_err(|_err| Error::InvalidFrameSize(TokenFrame::Signature))?;

        let expires_at: &[u8; EXPIRES_AT_SIZE] = bytes[EXPIRES_AT_OFFSET.0 .. EXPIRES_AT_OFFSET.1].try_into()
            .map_err(|_err| Error::InvalidFrameSize(TokenFrame::ExpiresAt))?;

        let audience: &[u8; AUDIENCE_SIZE] = bytes[AUDIENCE_OFFSET.0 .. AUDIENCE_OFFSET.1].try_into()
            .map_err(|_err| Error::InvalidFrameSize(TokenFrame::Audience))?;

        Ok(
            Token{
                signature,
                expires_at: u32::from_be_bytes(*expires_at),
                audience,
            }
          )
    }
}
