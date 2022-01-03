pub mod commands;
mod traits;
mod types;

use serde::Deserialize;
use serde::Serialize;
pub use traits::Trait;
pub use types::Type;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "command", content = "params", rename_all = "camelCase")]
#[non_exhaustive]
pub enum Command {
    #[serde(rename = "action.devices.commands.BrightnessAbsolute")]
    BrightnessAbsolute(commands::BrightnessAbsolute),
    #[serde(rename = "action.devices.commands.BrightnessRelative")]
    BrightnessRelative(commands::BrightnessRelative),
    #[serde(rename = "action.devices.commands.ColorAbsolute")]
    ColorAbsolute(commands::ColorAbsolute),
    #[serde(rename = "action.devices.commands.OnOff")]
    OnOff(commands::OnOff),
    #[serde(rename = "action.devices.commands.OpenClose")]
    OpenClose(commands::OpenClose),
}
