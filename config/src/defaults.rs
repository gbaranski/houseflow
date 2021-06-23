pub fn localhost(port: u16) -> std::net::SocketAddr {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))
}

pub fn server_address() -> std::net::SocketAddr {
    localhost(server_port())
}

pub const fn server_port() -> u16 {
    6001
}
