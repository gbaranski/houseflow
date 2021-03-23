use serde::Deserialize;

#[derive(Deserialize)]
pub struct PayloadDevice {
    /// Device ID, as per the ID provided in SYNC.
    pub id: String,
}

#[derive(Deserialize)]
pub struct Payload {
    /// List of device target and command pairs.
    pub devices: Vec<QueryPayloadDevice>
}
