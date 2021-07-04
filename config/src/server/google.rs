use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    ///  OAuth2 Client ID identifying Google to your service
    pub client_id: String,

    /// OAuth2 Client Secret assigned to the Client ID which identifies Google to you
    pub client_secret: String,

    /// Google Project ID
    pub project_id: String,
}
