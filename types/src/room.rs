use crate::structure;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

pub type ID = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Room {
    pub id: ID,
    pub structure_id: structure::ID,
    pub name: String,
}
