mod common;

use google_smart_home::device;
use google_smart_home::sync::response;
use google_smart_home::sync::response::Response;
use google_smart_home::Request;
use google_smart_home::RequestInput;
use serde_json::json;

#[test]
fn sync_request() {
    common::two_way_serde(
        include_str!("json/sync/request.json"),
        Request {
            request_id: String::from("ff36a3cc-ec34-11e6-b1a0-64510650abcf"),
            inputs: [RequestInput::Sync].to_vec(),
        },
    );
}

#[test]
fn sync_response() {
    common::two_way_serde(
        include_str!("json/sync/response.json"),
        Response {
            request_id: String::from("ff36a3cc-ec34-11e6-b1a0-64510650abcf"),
            payload: response::Payload {
                agent_user_id: String::from("1836.15267389"),
                error_code: None,
                debug_string: None,
                devices: [
                    response::PayloadDevice {
                        id: String::from("123"),
                        device_type: device::Type::Outlet,
                        traits: [device::Trait::OnOff].to_vec(),
                        name: response::PayloadDeviceName {
                            default_names: Some([String::from("My Outlet 1234")].to_vec()),
                            name: String::from("Night light"),
                            nicknames: Some([String::from("wall plug")].to_vec()),
                        },
                        will_report_state: false,
                        notification_supported_by_agent: false,
                        room_hint: Some(String::from("kitchen")),
                        device_info: Some(response::PayloadDeviceInfo {
                            manufacturer: Some(String::from("lights-out-inc")),
                            model: Some(String::from("hs1234")),
                            hw_version: Some(String::from("3.2")),
                            sw_version: Some(String::from("11.4")),
                        }),
                        attributes: Default::default(),
                        custom_data: Some(
                            json!({
                                "fooValue": 74,
                                "barValue": true,
                                "bazValue": "foo"
                            })
                            .as_object()
                            .unwrap()
                            .to_owned(),
                        ),
                        other_device_ids: Some(
                            [response::PayloadOtherDeviceID {
                                agent_id: None,
                                device_id: String::from("local-device-id"),
                            }]
                            .to_vec(),
                        ),
                    },
                    response::PayloadDevice {
                        id: String::from("456"),
                        device_type: device::Type::Light,
                        traits: [
                            device::Trait::OnOff,
                            device::Trait::Brightness,
                            device::Trait::ColorSetting,
                        ]
                        .to_vec(),
                        name: response::PayloadDeviceName {
                            default_names: Some(
                                [String::from("lights out inc. bulb A19 color hyperglow")].to_vec(),
                            ),
                            name: String::from("lamp1"),
                            nicknames: Some([String::from("reading lamp")].to_vec()),
                        },
                        will_report_state: false,
                        notification_supported_by_agent: false,
                        room_hint: Some(String::from("office")),
                        device_info: Some(response::PayloadDeviceInfo {
                            manufacturer: Some(String::from("lights out inc.")),
                            model: Some(String::from("hg11")),
                            hw_version: Some(String::from("1.2")),
                            sw_version: Some(String::from("5.4")),
                        }),
                        attributes: response::Attributes {
                            color_model: Some(response::ColorModel::Rgb),
                            color_temperature_range: Some(response::ColorTemperatureRange {
                                temperature_min_k: 2000,
                                temperature_max_k: 9000,
                            }),
                            command_only_color_setting: Some(false),
                            ..Default::default()
                        },
                        custom_data: Some(
                            json!({
                                "fooValue": 12,
                                "barValue": false,
                                "bazValue": "bar"
                            })
                            .as_object()
                            .unwrap()
                            .to_owned(),
                        ),
                        other_device_ids: None,
                    },
                ]
                .to_vec(),
            },
        },
    );
}
