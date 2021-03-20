#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod token;
mod payload;
mod signature;

pub use payload::Payload;
pub use signature::Signature;
pub use token::Token;

pub trait SizedFrame {
    const SIZE: usize;
}

