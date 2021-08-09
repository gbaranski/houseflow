use std::time::Instant;

use crate::CommandContext;
use async_trait::async_trait;
use houseflow_types::{
    fulfillment::execute, lighthouse::proto, DeviceCommand, DeviceID, DeviceStatus,
};

pub struct Command {
    pub device_id: DeviceID,
    pub command: DeviceCommand,
    pub params: serde_json::Map<String, serde_json::Value>,
    pub n: usize,
}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        let access_token = ctx.access_token().await?;
        let devices = match ctx.devices.get().await {
            Ok(devices) => devices,
            Err(szafka::Error::OpenFileError(err)) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(anyhow::Error::msg(
                        "no cached devices found, try `houseflow fulfillment sync`",
                    ))
                }
                _ => return Err(err.into())
            },
            Err(err) => return Err(err.into()),
        };
        let device = devices
            .iter()
            .find(|device| device.id == self.device_id)
            .ok_or_else(|| {
                anyhow::Error::msg(
                    "device not found, try `houseflow fulfillment sync` to fetch new devices",
                )
            })?;
        let supported_commands = device
            .traits
            .iter()
            .flat_map(|device_trait| device_trait.commands());
        let is_supported = supported_commands
            .clone()
            .any(|command| command == self.command);

        if !is_supported {
            return Err(anyhow::Error::msg(format!(
                "command is not supported by the device\nsupported commands: {}",
                supported_commands
                    .map(|command| command.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            )));
        }

        let execute_frame = proto::execute::Frame {
            id: rand::random(),
            command: self.command.clone(),
            params: self.params,
        };
        let request = execute::Request {
            device_id: self.device_id.clone(),
            frame: execute_frame,
        };

        use std::sync::Arc;
        let houseflow_api = Arc::new(ctx.houseflow_api().await?.to_owned());
        let access_token = &access_token;
        let request = &request;

        let futures = (0..self.n)
            .map(|i| {
                let request = request.clone();
                let access_token = access_token.clone();
                let houseflow_api = houseflow_api.clone();
                tokio::spawn(async move {
                    tracing::debug!("Executing request with index of {}", i);
                    let before = Instant::now();
                    let response = houseflow_api
                        .execute(&access_token, &request)
                        .await
                        .unwrap()
                        .unwrap();
                    let latency = Instant::now().duration_since(before).as_millis();
                    match response.frame.status {
                        DeviceStatus::Success => {
                            println!("✔ Device responded with success after {}ms!", latency)
                        }
                        DeviceStatus::Error(err) => {
                            println!("❌ Device responded with error! Error: {}", err)
                        }
                    };
                    latency
                })
            })
            .collect::<Vec<_>>();

        let mut latencies = futures::future::try_join_all(futures).await?;
        latencies.sort_unstable();
        if self.n > 0 {
            let average = Iterator::sum::<u128>(latencies.iter()) / latencies.len() as u128;
            let percentile =
                |val| latencies[(latencies.len() as f32 * (val as f32 * 0.01)) as usize];
            println!(
                "{} requests sent. Avg latency: {}ms, 50th percentile: {}ms, 95th percentile: {}ms",
                latencies.len(),
                average,
                percentile(50),
                percentile(95),
            );
        }

        // let mut futures = (0..self.n)
        //     .map(|i| async move {
        //     })
        //     .collect::<futures::stream::FuturesUnordered<_>>();
        // use futures::StreamExt;

        Ok(())
    }
}
