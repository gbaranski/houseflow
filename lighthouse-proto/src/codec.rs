use crate::{frame::Frame, Opcode};
use bytes::{Buf, BufMut, BytesMut};
use strum::IntoEnumIterator;
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug)]
pub enum Error {
    InvalidOpcode(u8),
    IOError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(item: std::io::Error) -> Error {
        Error::IOError(item)
    }
}

pub struct FrameCodec {}

impl Decoder for FrameCodec {
    type Item = Frame;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        const MIN_SIZE: usize = std::mem::size_of::<u8>();

        if src.len() < MIN_SIZE {
            return Ok(None);
        }

        let opcode = src.get_u8();
        let opcode = Opcode::iter()
            .find(|v| *v as u8 == opcode)
            .ok_or(Error::InvalidOpcode(opcode))?;

        let frame = match opcode {
            Opcode::Connect => {
                let mut client_id = [0; 16];
                src.copy_to_slice(&mut client_id[..]);

                Frame::Connect { client_id }
            }
            Opcode::ConnACK => Frame::ConnACK {
                response_code: src.get_u8(),
            },
        };

        Ok(Some(frame))
    }
}

impl Encoder<Frame> for FrameCodec {
    type Error = Error;

    fn encode(&mut self, item: Frame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            Frame::Connect { client_id } => {
                dst.put_u8(Opcode::Connect as u8);
                dst.put_slice(&client_id[..]);
            }
            Frame::ConnACK { response_code } => {
                dst.put_u8(Opcode::ConnACK as u8);
                dst.put_u8(response_code);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    fn test_frame_codec(frame: Frame) {
        let mut codec = FrameCodec {};
        let mut bytes = BytesMut::new();
        codec.encode(frame, &mut bytes).unwrap();

        let decoded_frame = codec.decode(&mut bytes).unwrap().unwrap();

        assert_eq!(frame, decoded_frame);
    }

    #[test]
    fn test_connect_codec() {
        let frame = Frame::Connect {
            client_id: random(),
        };
        test_frame_codec(frame)
    }

    #[test]
    fn test_connack_codec() {
        let frame = Frame::ConnACK {
            response_code: random(),
        };
        test_frame_codec(frame)
    }
}
