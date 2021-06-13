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
        use frame::*;
        use Opcode::*;

        let opcode: Opcode = buf
            .get_u8()
            .try_into()
            .map_err(|_| DecodeError::InvalidField { field: "opcode" })?;
        let frame: Self = match opcode {
            NoOperation => no_operation::Frame::decode(buf)?.into(),
            State => state::Frame::decode(buf)?.into(),
            StateCheck => state_check::Frame::decode(buf)?.into(),
            Execute => execute::Frame::decode(buf)?.into(),
            ExecuteResponse => execute_response::Frame::decode(buf)?.into(),
        };
        Ok(frame)
    }
}

impl Encoder for Frame {
    fn encode(&self, buf: &mut impl BufMut) {
        use Frame::*;

        buf.put_u8(self.opcode() as u8);
        match self {
            NoOperation(frame) => frame.encode(buf),
            State(frame) => frame.encode(buf),
            StateCheck(frame) => frame.encode(buf),
            Execute(frame) => frame.encode(buf),
            ExecuteResponse(frame) => frame.encode(buf),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    use rand::random;

    fn test_frame_codec<F>(frame: F)
    where
        F: Decoder + Encoder + std::fmt::Debug + PartialEq + Eq + Into<Frame> + Clone,
    {
        let mut buf = BytesMut::new();
        let full_frame: Frame = frame.clone().into();
        full_frame.encode(&mut buf);
        let frame_decoded = Frame::decode(&mut buf).expect("failed decoding");
        assert_eq!(frame.into(), frame_decoded);
        assert_eq!(buf.remaining(), 0);

        // Test with random sizes of random data
        for i in 1..512 {
            let buf: Vec<u8> = (0..i).map(|_| random()).collect();
            let buf: &[u8] = buf.as_ref();
            let buf = BytesMut::from(buf);
            let _ = F::decode(&mut buf.clone());
            let _ = Frame::decode(&mut buf.clone());
        }
    }

    #[test]
    fn no_operation() {
        let frame = frame::no_operation::Frame {};
        test_frame_codec(frame)
    }

    #[test]
    fn state() {
        let state = r#"
                {
                    "on": true,
                    "online": true,
                    "openPercent": 20
                }
               "#;
        let frame = frame::state::Frame {
            state: serde_json::from_str(state).unwrap(),
        };
        test_frame_codec(frame)
    }

    #[test]
    fn state_check() {
        let frame = frame::state_check::Frame {};
        test_frame_codec(frame)
    }

    #[test]
    fn execute() {
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
        test_frame_codec(frame)
    }

    #[test]
    fn execute_response() {
        let state = r#"
            {
                "on": true,
                "online": true,
                "openPercent": 20
            }
            "#;
        let frame = frame::execute_response::Frame {
            id: random(),
            status: random(),
            error: random(),
            state: serde_json::from_str(state).unwrap(),
        };
        test_frame_codec(frame)
    }
}
