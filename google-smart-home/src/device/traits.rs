use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
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
