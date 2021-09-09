use serde::Serialize;
use crate::structure;
use uuid::Uuid;
use serde::Deserialize;

pub type ID = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Room {
    pub id: ID,
    pub structure_id: structure::ID,
    pub name: String,
}

