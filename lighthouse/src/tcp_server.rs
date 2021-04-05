use uuid::Uuid;
use std::collections::HashMap;

pub type DeviceSessions = HashMap<Uuid, DeviceSession>;

pub struct DeviceSession {
    pub device_id: Uuid,
}
