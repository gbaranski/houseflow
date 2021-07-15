use async_trait::async_trait;
use houseflow_types::{DeviceCommand, DeviceError, DeviceStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteParams {
    NoOperation(()),
    OnOff { on: bool },
}

impl super::ExecuteParams for ExecuteParams {}

pub trait OnOffHook: Fn(&mut State, bool) -> DeviceStatus + Send {}

impl std::fmt::Debug for dyn OnOffHook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[Function]")
    }
}
impl<T> OnOffHook for T where T: Fn(&mut State, bool) -> DeviceStatus + Send {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    pub on: bool,
}

#[derive(Debug)]
pub struct Hooks {
    on_off: Box<dyn OnOffHook>,
}

impl Default for Hooks {
    fn default() -> Self {
        Self {
            on_off: Box::new(|state, on| {
                tracing::info!("Changing `on` to {0}", on);
                state.on = on;

                DeviceStatus::Success
            }),
        }
    }
}

#[derive(Default, Debug)]
pub struct Device {
    state: State,
    hooks: Hooks,
}

impl Device {
    pub fn new(on_off_hook: impl OnOffHook + 'static) -> Self {
        Self {
            state: State { on: false },
            hooks: Hooks {
                on_off: Box::new(on_off_hook),
            },
        }
    }
}

#[async_trait]
impl super::Device<ExecuteParams> for Device {
    async fn on_execute(
        &mut self,
        command: DeviceCommand,
        params: ExecuteParams,
    ) -> anyhow::Result<DeviceStatus> {
        let result = match command {
            DeviceCommand::OnOff => match params {
                ExecuteParams::OnOff { on } => (self.hooks.on_off)(&mut self.state, on),
                _ => DeviceStatus::Error(DeviceError::InvalidParameters),
            },
            _ => DeviceStatus::Error(DeviceError::FunctionNotSupported),
        };
        Ok(result)
    }

    fn state(&self) -> serde_json::Map<String, serde_json::Value> {
        serde_json::to_value(&self.state)
            .unwrap()
            .as_object()
            .unwrap()
            .clone()
    }
}
