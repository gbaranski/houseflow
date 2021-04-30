use crate::frame::{self, Frame, Opcode};
use bytes::{Buf, BufMut, BytesMut};
use std::convert::{TryFrom, TryInto};
use tokio_util::codec::{Decoder, Encoder};

/// Max size of JSON-encoded field in frame
const MAX_JSON_LEN: usize = 1024;

#[derive(Debug)]
pub enum Error {
    InvalidField(&'static str, Box<dyn std::fmt::Debug>),
    FieldTooBig(usize),
    FieldNotNullTerminated,
    IOError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Error::*;
        let msg = match self {
            InvalidField(field, received) => {
                format!("Invalid `{}`: `{:?}`", field, received)
            }
            FieldTooBig(size) => {
                format!(
                    "Received too big field, aborting decoding it, size: `{}`",
                    size
                )
            }
            FieldNotNullTerminated => {
                format!("Field was not NULL terminated")
            }
            IOError(err) => format!("IOError: {}", err),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(item: std::io::Error) -> Error {
        Error::IOError(item)
    }
}

pub struct FrameCodec {}

impl FrameCodec {
    pub fn new() -> Self {
        Self {}
    }
}

impl Decoder for FrameCodec {
    type Item = Frame;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        const MIN_SIZE: usize = std::mem::size_of::<u8>();

        if src.len() < MIN_SIZE {
            return Ok(None);
        }

        let opcode = src.get_u8();

        let opcode =
            Opcode::try_from(opcode).or(Err(Error::InvalidField("opcode", Box::new(opcode))))?;

        let frame = match opcode {
            Opcode::NoOperation => {
                return Err(Error::InvalidField("opcode", Box::new(opcode)));
            }
            Opcode::Connect => {
                let mut client_id = [0; 16];
                src.copy_to_slice(&mut client_id[..]);

                let frame = frame::connect::Frame {
                    client_id: client_id.into(),
                };

                Frame::Connect(frame)
            }
            Opcode::ConnACK => {
                let response_code = src.get_u8();

                let frame = frame::connack::Frame {
                    response_code: response_code.try_into().map_err(|_| {
                        Error::InvalidField("ConnACK.Response", Box::new(response_code))
                    })?,
                };

                Frame::ConnACK(frame)
            }
            Opcode::Execute => {
                let id = src.get_u32();
                let command = src.get_u16();
                let command = frame::execute::Command::try_from(command)
                    .map_err(|_| Error::InvalidField("Execute.Command", Box::new(command)))?;
                let params_len = src
                    .iter()
                    .position(|v| *v == b'\0')
                    .ok_or(Error::FieldNotNullTerminated)?;
                if params_len > MAX_JSON_LEN {
                    return Err(Error::FieldTooBig(params_len));
                }
                let params_bytes = src.copy_to_bytes(params_len);
                let params = serde_json::from_slice(&params_bytes)
                    .map_err(|err| Error::InvalidField("Execute.Params", Box::new(err)))?;

                let frame = frame::execute::Frame {
                    id,
                    command,
                    params,
                };

                Frame::Execute(frame)
            }
            Opcode::ExecuteResponse => {
                let id = src.get_u32();
                let response_code = src.get_u8();
                let response_code = frame::execute_response::ResponseCode::try_from(response_code)
                    .map_err(|_| {
                        Error::InvalidField("ExecuteResponse.ResponseCode", Box::new(response_code))
                    })?;

                let error = src.get_u16();
                let error = frame::execute_response::Error::try_from(error)
                    .map_err(|_| Error::InvalidField("ExecuteResponse.Error", Box::new(error)))?;
                let state_len = src
                    .iter()
                    .position(|v| *v == b'\0')
                    .ok_or(Error::FieldNotNullTerminated)?;
                if state_len > MAX_JSON_LEN {
                    return Err(Error::FieldTooBig(state_len));
                }
                let state_bytes = src.copy_to_bytes(state_len);
                let state = serde_json::from_slice(&state_bytes).unwrap();

                let frame = frame::execute_response::Frame {
                    id,
                    response_code,
                    state,
                    error,
                };

                Frame::ExecuteResponse(frame)
            }
        };

        Ok(Some(frame))
    }
}

impl Encoder<Frame> for FrameCodec {
    type Error = Error;

    fn encode(&mut self, item: Frame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let opcode = item.opcode();
        dst.put_u8(opcode as u8);

        match item {
            Frame::NoOperation => {
                let opcode = Frame::NoOperation.opcode();
                return Err(Error::InvalidField("opcode", Box::new(opcode)));
            }
            Frame::Connect(frame) => {
                let client_id: [u8; 16] = frame.client_id.into();
                dst.put_slice(&client_id[..]);
            }
            Frame::ConnACK(frame) => {
                dst.put_u8(frame.response_code as u8);
            }
            Frame::Execute(frame) => {
                dst.put_u32(frame.id);
                dst.put_u16(frame.command as u16);
                let params = serde_json::to_vec(&frame.params).unwrap();
                dst.put_slice(&params);
                dst.put_u8(b'\0');
            }
            Frame::ExecuteResponse(frame) => {
                dst.put_u32(frame.id);
                dst.put_u8(frame.response_code as u8);
                dst.put_u16(frame.error as u16);
                let state = serde_json::to_vec(&frame.state).unwrap();
                dst.put_slice(&state);
                dst.put_u8(b'\0');
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
        codec.encode(frame.clone(), &mut bytes).unwrap();

        let decoded_frame = codec.decode(&mut bytes).unwrap().unwrap();

        assert_eq!(frame, decoded_frame);
    }

    #[test]
    fn test_connect_codec() {
        let frame = frame::connect::Frame {
            client_id: random(),
        };
        test_frame_codec(Frame::Connect(frame))
    }

    #[test]
    fn test_connack_codec() {
        let frame = frame::connack::Frame {
            response_code: random(),
        };
        test_frame_codec(Frame::ConnACK(frame))
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
