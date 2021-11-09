use futures::StreamExt;
use hap::server::Server;
use houseflow_config::hub::Config;
use houseflow_config::hub::DeviceType;
use houseflow_types::device;
use mijia::MijiaEvent;
use mijia::MijiaSession;
use std::collections::HashMap;

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

        // Setup Homekit
        {
            use hap::accessory::AccessoryCategory;
            use hap::characteristic::CharacteristicCallbacks;
            use hap::storage::FileStorage;
            use hap::storage::Storage;
            use hap::MacAddress;
            use hap::Pin;

            let mut storage = FileStorage::current_dir().await?;
            let config = match storage.load_config().await {
                Ok(mut config) => {
                    config.redetermine_local_ip();
                    storage.save_config(&config).await?;
                    config
                }
                Err(_) => hap::Config {
                    pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3])?,
                    name: "Acme Lightbulb".into(),
                    device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
                    category: AccessoryCategory::Lightbulb,
                    ..Default::default()
                },
            };

            storage.save_config(&config).await?;
            let server = hap::server::IpServer::new(config, storage).await?;

            use hap::accessory::lightbulb::LightbulbAccessory;
            use hap::accessory::AccessoryInformation;

            let mut lightbulb = LightbulbAccessory::new(1, AccessoryInformation {
                name: "Acme Lightbulb".into(),
                ..Default::default()
            })?;
            lightbulb.lightbulb.power_state.on_read(Some(|| {
                println!("power_state characteristic read");
                Ok(None)
            }));
            
            lightbulb.lightbulb.power_state.on_update(Some(|current_val: &bool, new_val: &bool| {
                println!("power_state characteristic updated from {} to {}", current_val, new_val);
                Ok(())
            }));
            server.add_accessory(lightbulb).await?;

            tokio::spawn(async move { server.run_handle().await.unwrap() });
        }

        discover_xiaomi_mijia_fut.await?;

        Ok(())
    }

    async fn discover_xiaomi_mijia(&self) -> Result<(), anyhow::Error> {
        let mut devices = HashMap::<mijia::bluetooth::DeviceId, device::ID>::new();
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

        let (_, session) = MijiaSession::new().await?;
        session.bt_session.start_discovery().await?;
        let sensors = session.get_sensors().await?;
        for sensor in sensors {
            let device = find_device_by_mac(sensor.mac_address.to_string().as_str());
            if let Some(device) = device {
                tracing::info!(mac = %sensor.mac_address, id = %device.id, "discovered, connecting");
                if let Err(err) = session.bt_session.connect(&sensor.id).await {
                    tracing::info!(mac = %sensor.mac_address, id = %device.id, "connect failed due to {}", err);
                } else {
                    tracing::info!(mac = %sensor.mac_address, id = %device.id, "successfully connected");
                    session.start_notify_sensor(&sensor.id).await?;
                    devices.insert(sensor.id, device.id);
                }
            } else {
                tracing::info!(mac = %sensor.mac_address, "discovered, skipping");
            }
        }
        let mut stream = session.event_stream().await?;
        while let Some(event) = stream.next().await {
            tracing::debug!("received event = {:?}", event);
            match event {
                MijiaEvent::Discovered { id } => {
                    tracing::info!("discovered: {}", id);
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
