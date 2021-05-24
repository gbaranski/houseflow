use url::Url;

pub struct Fulfillment {
    url: Url,
}

impl Fulfillment {
    pub fn new(url: Url) -> Self {
        Self { url }
    }
}
