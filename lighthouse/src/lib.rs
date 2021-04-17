#[forbid(unsafe_code)] 
pub mod server;

pub enum Error {
    /// Indicates that client was not connected at the time the request was sent.
    ClientNotConnected,
    
    /// This error is ca
    Other,
}
