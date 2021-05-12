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
        .map(|(index, device)| (&device.id, index));

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
    let text_view = TextView::new("Select action");
    let dialog_title = format!("Selected: {}", device.id);
    let dialog = Dialog::around(text_view)
        .title(dialog_title)
        .button("Cancel", |s| s.quit());

    siv.add_layer(dialog);
}
