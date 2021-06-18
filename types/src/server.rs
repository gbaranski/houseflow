use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerSecrets {
    /// Key used to sign refresh tokens. Must be secret and should be farily random.
    pub refresh_key: String,

    /// Key used to sign access tokens. Must be secret and should be farily random.
    pub access_key: String,

    /// Salt used with hashing passwords
    pub password_salt: String,
}
