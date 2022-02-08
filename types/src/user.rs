use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

pub type ID = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// Unique ID of the user
    pub id: ID,
    /// Name of the user
    pub username: String,
    /// Email of the user
    pub email: lettre::Address,
    /// True if the user is admin.
    pub admin: bool,
}
