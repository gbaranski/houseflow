use url::Url;
use reqwest::Client;
use houseflow_types::{Device, UserAgent};
use houseflow_token::Token;

pub struct Fulfillment {
    url: Url,
    refresh_token: Token,
    access_token: Token,
}

impl Fulfillment {
    pub fn new(url: Url, refresh_token: Token) -> Self {
        Self { url, refresh_token }
    }

    async fn refresh_token(&mut self) -> bool {
        if !self.access_token.has_expired() {
            return false
        }
        let client = Client::new();
        let query = format!("grant_type=refresh_token&refresh_token={}", self.refresh_token.to_base64());
        let url = self.url
            .join("token")
            .unwrap()
            .set_query(Some(&query));
        let response =  client.post(self.url.join("token").unwrap());


        true
    }

    pub async fn sync(&mut self) -> Vec<Device> {

    }
}
