type DeviceID = String;

pub struct Device {
    pub id: DeviceID,
}
fn generate_random_device_id() -> DeviceID {
    let bytes: [u8; 16] = rand::random();
    hex::encode(bytes)
}
/// Creates a infinite Iterator of fake devices
fn get_devices() -> impl Iterator<Item = Device> {
    std::iter::repeat_with(|| Device {
        id: generate_random_device_id(),
    })
}
