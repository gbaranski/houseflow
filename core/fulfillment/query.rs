use crate::{ClientCommandState, Command};
use async_trait::async_trait;
use houseflow_types::{
    fulfillment::query, lighthouse::proto, DeviceID,
};

use clap::Clap;

#[derive(Clap)]
pub struct QueryCommand {
    pub device_id: DeviceID,
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for QueryCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        let access_token = state.access_token().await?;
        let devices = state.devices.get().await?;
        let _ =  devices
            .iter()
            .find(|device| device.id == self.device_id)
            .ok_or_else(|| {
                anyhow::Error::msg(
                    "device not found, try `houseflow fulfillment sync` to fetch new devices",
                )
            })?;

        let query_frame = proto::query::Frame {};
        let request = query::Request {
            device_id: self.device_id.clone(),
            frame: query_frame,
        };
        let response = state
            .houseflow_api
            .query(&access_token, &request)
            .await??;

        println!("Device responded with state: {:#?}", response.frame.state);

        Ok(())
    }
}
