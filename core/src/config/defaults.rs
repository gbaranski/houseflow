use url::Url;

pub fn base_url() -> Url {
    Url::parse(&format!("http://{}:{}", host(), port())).unwrap()
}

pub fn base_url_websockets() -> Url {
    Url::parse(&format!("ws://{}:{}", host(), port())).unwrap()
}

pub fn host() -> String {
    String::from("127.0.0.1")
}

pub fn port() -> u16 {
    6001
}
