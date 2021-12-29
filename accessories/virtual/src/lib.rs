use async_trait::async_trait;
use houseflow_accessory_hal::Accessory;
use houseflow_types::accessory;
use tokio::sync::Mutex;

pub struct VirtualAccessory {
    state: Mutex<accessory::State>,
}

impl VirtualAccessory {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(accessory::State {
                temperature: None,
                humidity: None,
                on: None,
                open_percent: Some(0),
                battery_percent: None,
                battery_voltage: None,
            }),
        }
    }
}

#[async_trait]
impl Accessory for VirtualAccessory {
    async fn execute(
        &self,
        command: accessory::Command,
    ) -> Result<accessory::Status, houseflow_accessory_hal::Error> {
        let mut state = self.state.lock().await;
        match command {
            accessory::Command::OnOff(accessory::commands::OnOff { on }) => {
                state.on = Some(on);
            }
            accessory::Command::OpenClose(accessory::commands::OpenClose { open_percent }) => {
                state.open_percent = Some(open_percent);
            }
            _ => unimplemented!(),
        }
        Ok(accessory::Status::Success)
    }

    async fn state(&self) -> Result<accessory::State, houseflow_accessory_hal::Error> {
        Ok(self.state.lock().await.to_owned())
    }
}
