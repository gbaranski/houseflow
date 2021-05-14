
/// Used to identify the device
#[derive(Debug, Clone)]
pub struct DeviceID {
    inner: [u8; 16],
}

#[derive(Debug, Clone)]
pub struct Device {
    pub id: DeviceID,
}



use std::fmt;

impl fmt::Display for DeviceID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.inner))
    }
}


use rand::distributions;

impl distributions::Distribution<DeviceID> for distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> DeviceID {
        DeviceID { inner: rng.gen() }
    }
}


/// Creates a infinite Iterator of fake devices
pub fn get_devices() -> impl Iterator<Item = Device> {
    std::iter::repeat_with(|| Device { id: rand::random() })
}

