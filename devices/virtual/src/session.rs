use futures_util::{SinkExt, StreamExt};
use houseflow_types::{DeviceID, DevicePassword};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::connect_async;
use url::Url;

pub struct Options {
    pub url: Url,
    pub id: DeviceID,
    pub password: DevicePassword,
}

pub struct Session {
    opts: Options,
}

impl Session {
    pub fn new(opts: Options) -> Self {
        Self { opts }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        let http_request = http::Request::builder()
            .uri(self.opts.url.to_string())
            .header(http::header::AUTHORIZATION, format!("Basic {}:{}", self.opts.id, self.opts.password))
            .body(())
            .unwrap();

        let (stream, response) = tokio_tungstenite::connect_async(http_request).await?;
        let (mut sender, receiver) = stream.split();

        let mut ticks: usize = 0;
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            sender.send(tungstenite::Message::Text(format!("Tick: {}", ticks))).await?;
            ticks += 1;
        }
    }
}
