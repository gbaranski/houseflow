use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Hash)]
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
