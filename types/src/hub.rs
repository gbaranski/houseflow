use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

pub type ID = Uuid;
pub type Password = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Hub {
    pub id: ID,
    pub name: String,
    pub password_hash: Option<String>,
}
