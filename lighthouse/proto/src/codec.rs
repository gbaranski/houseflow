use crate::frame::{self, Frame, Opcode};
use bytes::{Buf, BufMut};
use std::convert::TryInto;
use std::mem::size_of;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Invalid JSON: `{0}`")]
    InvalidJSON(#[from] serde_json::Error),

    #[error("Frame has invalid field `{field}`")]
    InvalidField { field: &'static str },

    #[error("Invalid size, expected: `{received}`, received: `{received}`")]
    InvalidSize { expected: usize, received: usize },
}

pub trait Decoder {
    const MIN_SIZE: usize;
    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub trait Encoder {
    fn encode(&self, buf: &mut impl BufMut);
}

impl Decoder for Frame {
    const MIN_SIZE: usize = size_of::<Opcode>();

    fn decode(buf: &mut impl Buf) -> Result<Self, DecodeError> {
        let opcode: Opcode = buf
            .get_u8()
            .try_into()
            .map_err(|_| DecodeError::InvalidField { field: "opcode" })?;
        let frame = match opcode {
            Opcode::NoOperation => Frame::NoOperation,
            Opcode::Execute => Frame::Execute(frame::execute::Frame::decode(buf)?),
            Opcode::ExecuteResponse => {
                Frame::ExecuteResponse(frame::execute_response::Frame::decode(buf)?)
            }
        };

        Ok(frame)
    }
}

impl Encoder for Frame {
    fn encode(&self, buf: &mut impl BufMut) {
        buf.put_u8(self.opcode() as u8);
        match self {
            Frame::NoOperation => {}
            Frame::Execute(frame) => {
                frame.encode(buf);
            }
            Frame::ExecuteResponse(frame) => {
                frame.encode(buf);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    use rand::random;

    fn test_frame_codec(frame: Frame) {
        let mut buf = BytesMut::new();
        frame.encode(&mut buf);
        let frame_decoded = Frame::decode(&mut buf).expect("failed decoding");
        assert_eq!(frame, frame_decoded);
        assert_eq!(buf.remaining(), 0);
    }

    #[test]
    fn test_execute_codec() {
        let params = r#"
            {
                "on": true,
                "online": true,
                "openPercent": 20
            }
            "#;
        let frame = frame::execute::Frame {
            id: random(),
            command: random(),
            params: serde_json::from_str(params).unwrap(),
        };
        test_frame_codec(Frame::Execute(frame))
    }

    #[test]
    fn test_execute_response_codec() {
        let state = r#"
            {
                "on": true,
                "online": true,
                "openPercent": 20
            }
            "#;
        let frame = frame::execute_response::Frame {
            id: random(),
            response_code: random(),
            error: random(),
            state: serde_json::from_str(state).unwrap(),
        };
        test_frame_codec(Frame::ExecuteResponse(frame))
    }
}
