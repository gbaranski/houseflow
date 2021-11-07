use std::collections::HashMap;
use futures::StreamExt;
use houseflow_config::hub::Config;
use houseflow_config::hub::DeviceType;
use houseflow_types::device;
use mijia::MijiaEvent;
use mijia::MijiaSession;

#[derive(Clone)]
pub struct Hub {
    pub config: Config,
}

impl Hub {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        let discover_xiaomi_mijia_fut = tokio::spawn({
            let this = self.clone();
            async move {
                tracing::info!("start discovering xiaomi mijia devices");
                match this.discover_xiaomi_mijia().await {
                    Ok(_) => {
                        tracing::info!("discovering xiaomi mijia devices completed sucesfully")
                    }
                    Err(err) => {
                        tracing::info!("discovering xiaomi mijia devices failed due to {}", err);
                    }
                };
            }
        });

        discover_xiaomi_mijia_fut.await?;

        Ok(())
    }

    async fn discover_xiaomi_mijia(&self) -> Result<(), anyhow::Error> {
        let mut devices = HashMap::<mijia::bluetooth::DeviceId, device::ID>::new();
        let (_, session) = MijiaSession::new().await?;

        let find_device_by_mac = |expected_mac_address: &str| {
            self.config
                .devices
                .iter()
                .find(|device| match &device.r#type {
                    DeviceType::XiaomiMijia { mac_address } => {
                        mac_address.as_str() == expected_mac_address
                    }
                    _ => false,
                })
        };

        session.bt_session.start_discovery().await?;
        let mut stream = session.event_stream().await?;
        while let Some(event) = stream.next().await {
            tracing::debug!("received event = {:?}", event);
            match event {
                MijiaEvent::Discovered { id } => {
                    let device_info = session.bt_session.get_device_info(&id).await?;
                    let device = find_device_by_mac(device_info.mac_address.to_string().as_str());
                    if let Some(device) = device {
                        tracing::info!(mac = %device_info.mac_address, id = %device.id, "discovered, connecting");
                        if let Err(err) = session.bt_session.connect(&id).await {
                            tracing::info!(mac = %device_info.mac_address, id = %device.id, "connect failed due to {}", err);
                        } else {
                            tracing::info!(mac = %device_info.mac_address, id = %device.id, "successfully connected");
                            devices.insert(id, device.id);
                        }
                    } else {
                        tracing::info!(mac = %device_info.mac_address, "discovered, skipping");
                    }
                }
                MijiaEvent::Readings { id, readings } => {
                    let device_id = devices.get(&id).unwrap();
                    tracing::info!("readings from {} = {}", device_id, readings);
                }
                MijiaEvent::HistoryRecord { id, record } => {
                    let device_id = devices.get(&id).unwrap();
                    tracing::info!("new history record from {} = {}", device_id, record);
                }
                MijiaEvent::Disconnected { id } => {
                    let device_id = devices.get(&id).unwrap();
                    tracing::info!("{} disconnected", device_id);
                    devices.remove(&id);
                }
                _ => todo!(),
            };
        }
        Ok::<(), anyhow::Error>(())
    }
}
