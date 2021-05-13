/// Used to identify the device
struct DeviceID {
    inner: [u8; 16],
}

struct Device {
    pub id: DeviceID,
}

impl std::fmt::Display for DeviceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.inner))
    }
}

use rand::distributions;

impl distributions::Distribution<DeviceID> for distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> DeviceID {
        DeviceID {
            inner: rng.gen(),
        }
    }
}

/// Creates a infinite Iterator of fake devices
fn get_devices() -> impl Iterator<Item = Device> {
    std::iter::repeat_with(|| Device { id: rand::random() })
}

use cursive::Cursive;
use cursive::{view::View, views::SelectView};

/// Returns SelectView whichs shows all available devices to user
fn get_devices_select_view(
    devices: Vec<Device>,
    submit_callback: impl 'static + Fn(&mut Cursive, &Device),
) -> impl View {
    let devices_cursive_iter = devices
        .iter()
        .enumerate()
        .map(|(index, device)| (device.id.to_string(), index));

    let mut view = SelectView::new();
    view.add_all(devices_cursive_iter);
    view.set_on_submit(move |siv, index| {
        let device = devices.get(*index).unwrap();
        submit_callback(siv, device)
    });
    view
}

use cursive::views::{Dialog, TextView};

fn device_select_callback(siv: &mut Cursive, device: &Device) {
    let text_view = TextView::new("Select what to do with the device");
    let dialog_title = format!("Selected device: {}", device.id);
    let dialog = Dialog::around(text_view)
        .title(dialog_title)
        .button("Send command", |s| {
            s.pop_layer().unwrap();
        })
        .button("Cancel", |s| {
            s.pop_layer().unwrap();
        });

    siv.add_layer(dialog);
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut siv = cursive::default();
    let devices = get_devices().take(3).collect();
    let devices_select_view = get_devices_select_view(devices, device_select_callback);
    siv.add_layer(devices_select_view);
    siv.run();

    Ok(())
}
