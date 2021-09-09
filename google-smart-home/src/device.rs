use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "command", content = "params", rename_all = "camelCase")]
#[non_exhaustive]
pub enum Command {
    #[serde(rename = "action.devices.commands.OnOff")]
    OnOff(commands::OnOff),
    #[serde(rename = "action.devices.commands.OpenClose")]
    OpenClose(commands::OpenClose),
}

pub mod commands {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OnOff {
        pub on: bool,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OpenClose {
        #[serde(alias = "openPercent")]
        pub open_percent: u8,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Trait {
    #[serde(rename = "action.devices.traits.OnOff")]
    OnOff,
    #[serde(rename = "action.devices.traits.OpenClose")]
    OpenClose,
    #[serde(rename = "action.devices.traits.Brightness")]
    Brightness,
    #[serde(rename = "action.devices.traits.ColorSetting")]
    ColorSetting,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Type {
    #[serde(rename = "action.devices.types.GARAGE")]
    Garage,
    #[serde(rename = "action.devices.types.GATE")]
    Gate,
    #[serde(rename = "action.devices.types.LIGHT")]
    Light,
    #[serde(rename = "action.devices.types.OUTLET")]
    Outlet,
}
