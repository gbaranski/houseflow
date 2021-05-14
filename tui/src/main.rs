mod device;

use device::{get_devices, Device};

use cursive_async_view::AsyncView;
use cursive::{
    views::{Dialog, TextView, OnEventView, SelectView},
    direction::{Absolute, Direction},
    event::{Event, EventTrigger, EventResult},
    Cursive, View,
};

fn main() -> anyhow::Result<()> {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    let devices = get_devices().take(10).collect();
    let devices_select_view = get_devices_select_view(devices, submit_callback);

    // Create a dialog with devices select view
    siv.add_layer(Dialog::around(devices_select_view));

    // Starts the event loop.
    siv.run();

    Ok(())
}


fn send_command(siv: &mut Cursive, _device: &Device) {
    let client = reqwest::blocking::Client::new();
    let request = client.post("http://httpbin.org/delay/2");
    let async_view = AsyncView::new_with_bg_creator(
        siv,
        move || match request.send() {
            Ok(response) => Ok(response.status().to_string()),
            Err(err) => Err(err.to_string()),
        },
        TextView::new,
    );
    let async_view_width = siv.screen_size().x / 3;
    let async_view = Dialog::around(async_view.with_width(async_view_width)).button("Ok", |siv| {
        siv.pop_layer();
    });
    siv.add_layer(async_view);
}

fn submit_callback(siv: &mut Cursive, device: Device) {
    let text_view = TextView::new("Select what to do with the device");
    let dialog_title = format!("Selected device: {}", device.id);
    let dialog = Dialog::around(text_view)
        .title(dialog_title)
        .button("Send Command", move |siv| send_command(siv, &device))
        .button("Cancel", |siv| {
            siv.pop_layer();
        });
    let dialog = OnEventView::new(dialog)
        .on_pre_event_inner(
            EventTrigger::none()
                .or(Event::Char('h'))
                .or(Event::Char('k')),
            |siv, _| {
                siv.take_focus(Direction::Abs(Absolute::Left));
                Some(EventResult::Consumed(None))
            },
        )
        .on_pre_event_inner(
            EventTrigger::none()
                .or(Event::Char('l'))
                .or(Event::Char('j')),
            |siv, _| {
                siv.take_focus(Direction::Abs(Absolute::Right));
                Some(EventResult::Consumed(None))
            },
        );

    siv.add_layer(dialog);
}

/// Returns SelectView whichs shows all available devices to user
fn get_devices_select_view(
    devices: Vec<Device>,
    submit_callback: impl 'static + Fn(&mut Cursive, Device),
) -> impl View {
    let devices_cursive_iter = devices
        .iter()
        .enumerate()
        .map(|(index, device)| (device.id.to_string(), index));

    let mut view = SelectView::new();
    view.add_all(devices_cursive_iter);
    view.set_on_submit(move |siv, index| {
        let device = devices.get(*index).unwrap();
        submit_callback(siv, device.clone())
    });
    OnEventView::new(view)
        .on_pre_event_inner('k', |siv, _| {
            let cb = siv.select_up(1); // Move up
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |siv, _| {
            let cb = siv.select_down(1); // Move down
            Some(EventResult::Consumed(Some(cb)))
        })
}
