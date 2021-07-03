use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub state: serde_json::Map<String, serde_json::Value>,
}
