use crate::{ClientCommandState, Command};
use async_trait::async_trait;

use clap::Clap;

use houseflow_types::{admin::AddDeviceRequest, DeviceTrait, DeviceType, RoomID};
use semver::Version;
use std::collections::HashMap;

fn from_json<'de, T: serde::de::Deserialize<'de>>(v: &'de str) -> Result<T, serde_json::Error> {
    serde_json::from_str(v)
}

struct Traits {
    inner: Vec<DeviceTrait>,
}

impl From<Vec<DeviceTrait>> for Traits {
    fn from(inner: Vec<DeviceTrait>) -> Self {
        Self { inner }
    }
}

impl Into<Vec<DeviceTrait>> for Traits {
    fn into(self) -> Vec<DeviceTrait> {
        self.inner
    }
}

impl std::str::FromStr for Traits {
    type Err = <DeviceTrait as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|s| s.trim())
            .map(|s| DeviceTrait::from_str(s))
            .collect::<Result<Vec<_>, _>>()
            .map(|traits| Self::from(traits))
    }
}

#[derive(Clap)]
pub struct AddDeviceCommand {
    /// ID of the room to which the device belongs
    room_id: RoomID,

    /// Password used to authenticate the device
    password: String,

    /// Type of the device, e.g light
    device_type: DeviceType,

    /// List of traits that the device has
    traits: Traits,

    /// Name of the device
    name: String,

    /// True if the device will push state, false if use polling model
    #[clap(parse(try_from_str))]
    will_push_state: bool,

    /// Model of the device
    model: String,

    /// Hardware version of the device
    hw_version: Version,

    /// Software version of the device
    sw_version: Version,

    /// Additional attributes of the device
    #[clap(parse(try_from_str = from_json))]
    attributes: HashMap<String, Option<String>>,
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for AddDeviceCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        // TODO: try to simplify that
        let request = AddDeviceRequest {
            room_id: self.room_id.clone(),
            password: self.password,
            device_type: self.device_type,
            traits: self.traits.into(),
            name: self.name,
            will_push_state: self.will_push_state,
            model: self.model,
            hw_version: self.hw_version,
            sw_version: self.sw_version,
            attributes: self.attributes,
        };

        let access_token = state.access_token().await?;
        let response = state
            .houseflow_api
            .admin_add_device(&access_token, &request)
            .await?
            .into_result()?;

        log::info!("✔ Succesfully added device with ID: {}", response.device_id);

        Ok(())
    }
}
