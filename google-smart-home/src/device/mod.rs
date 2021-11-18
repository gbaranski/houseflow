pub mod commands;
mod traits;
mod types;

use serde::Deserialize;
use serde::Serialize;
pub use traits::Trait;
pub use types::Type;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "command", content = "params", rename_all = "camelCase")]
#[non_exhaustive]
pub enum Command {
    #[serde(rename = "action.devices.commands.OnOff")]
    OnOff(commands::OnOff),
    #[serde(rename = "action.devices.commands.OpenClose")]
    OpenClose(commands::OpenClose),
}
