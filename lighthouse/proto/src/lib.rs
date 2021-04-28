mod codec;
pub mod frame;
pub use codec::{Error as FrameCodecError, FrameCodec};

#[derive(Debug)]
pub enum Error {
    InvalidSize { expected: usize, received: usize },
    InvalidOpcode(u8),
}
