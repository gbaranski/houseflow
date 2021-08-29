use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    #[serde(default)]
    pub state: serde_json::Map<String, serde_json::Value>,
}
