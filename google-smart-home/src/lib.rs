//! Types for [Google Smart Home API](https://developers.google.com/assistant/smarthome)

pub mod device;
pub mod execute;
pub mod query;
pub mod sync;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub request_id: String,
    pub inputs: Vec<RequestInput>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "intent", content = "payload")]
pub enum RequestInput {
    #[serde(rename = "action.devices.SYNC")]
    Sync,
    #[serde(rename = "action.devices.QUERY")]
    Query(query::request::Payload),
    #[serde(rename = "action.devices.EXECUTE")]
    Execute(execute::request::Payload),
    #[serde(rename = "action.devices.DISCONNECT")]
    Disconnect,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Response {
    Sync(sync::response::Response),
    Query(query::response::Response),
    Execute(execute::response::Response),
    Disconnect,
}
