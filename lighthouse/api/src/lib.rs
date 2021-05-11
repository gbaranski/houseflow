use houseflow_types::DeviceID;
use lighthouse_proto::{command, command_response};
use lighthouse_types::DeviceError;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum SendError {
    #[error("Error with device: {0}")]
    DeviceError(#[from] DeviceError),

    #[error("Error when sending request: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub struct Lighthouse {
    pub url: Url,
}

impl Lighthouse {
    pub async fn send_command(
        &self,
        frame: command::Frame,
        device_id: DeviceID,
    ) -> Result<command_response::Frame, SendError> {
        let device_id_string = device_id.to_string();
        let url = self
            .url
            .join("command/")
            .unwrap()
            .join(&device_id_string)
            .unwrap();

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&frame)
            .send()
            .await?
            .json::<command_response::Frame>()
            .await?;
        Ok(response)
    }
}
