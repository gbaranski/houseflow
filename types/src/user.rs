use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use std::time::Duration;
use uuid::Uuid;

pub type ID = Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// Unique ID of the user
    pub id: ID,
    /// Name of the user
    pub username: String,
    /// Email of the user
    pub email: String,
    /// True if the user is admin.
    pub admin: bool,
    /// Homie controller for the user.
    #[serde(default)]
    pub homie: Option<Homie>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Homie {
    /// The hostname of the MQTT broker.
    pub host: String,
    /// The port of the MQTT broker.
    pub port: u16,
    /// Whether to use TLS for the MQTT broker connection.
    #[serde(default)]
    pub use_tls: bool,
    /// The username with which to authenticate to the MQTT broker, if any.
    #[serde(default)]
    pub username: Option<String>,
    /// The password with which to authenticate to the MQTT broker, if any.
    #[serde(default)]
    pub password: Option<String>,
    /// The client ID to use for the MQTT connection.
    pub client_id: String,
    /// The Homie base MQTT topic.
    #[serde(default = "default_homie_prefix")]
    pub homie_prefix: String,
    #[serde(
        deserialize_with = "de_duration_seconds",
        rename = "reconnect-interval-seconds"
    )]
    pub reconnect_interval: Duration,
}

fn default_homie_prefix() -> String {
    "homie".to_string()
}

/// Deserialize an integer as a number of seconds.
fn de_duration_seconds<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
    let seconds = u64::deserialize(d)?;
    Ok(Duration::from_secs(seconds))
}
