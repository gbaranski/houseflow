use std::net::IpAddr;
use std::net::Ipv4Addr;
use url::Url;

pub fn server_websocket_url() -> Url {
    let url = format!("ws://localhost:{}", server_port());
    Url::parse(&url).unwrap()
}

pub fn server_http_url() -> Url {
    let url = format!("http://localhost:{}", server_port());
    Url::parse(&url).unwrap()
}

pub const fn server_listen_address() -> IpAddr {
    IpAddr::V4(Ipv4Addr::LOCALHOST)
}

pub const fn server_port() -> u16 {
    6001
}

pub const fn server_port_tls() -> u16 {
    6002
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

pub fn clerk_path() -> std::path::PathBuf {
    data_home().join("clerk.sled")
}

pub fn smtp_port() -> u16 {
    587
}
