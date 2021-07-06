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

pub fn base_directories() -> xdg::BaseDirectories {
    xdg::BaseDirectories::with_prefix("houseflow").unwrap()
}

pub fn config_home() -> std::path::PathBuf {
    base_directories().get_config_home()
}

pub fn data_home() -> std::path::PathBuf {
    base_directories().get_data_home()
}

pub fn database_path() -> std::path::PathBuf {
    data_home().join("database.sqlite3")
}

pub fn token_store_path() -> std::path::PathBuf {
    data_home().join("token_store.sled")
}
