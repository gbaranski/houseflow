use crate::user;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

pub type ID = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Permission {
    pub structure_id: ID,
    pub user_id: user::ID,
    pub is_manager: bool,
}
