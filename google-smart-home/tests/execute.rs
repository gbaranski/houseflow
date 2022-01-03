mod common;

use google_smart_home::device::commands;
use google_smart_home::device::Command;
use google_smart_home::execute::request;
use google_smart_home::execute::response;
use google_smart_home::execute::response::Response;
use google_smart_home::Request;
use google_smart_home::RequestInput;
use serde_json::json;

#[test]
fn execute_request() {
    common::two_way_serde(
        include_str!("json/execute/request.json"),
        Request {
            request_id: String::from("ff36a3cc-ec34-11e6-b1a0-64510650abcf"),
            inputs: [RequestInput::Execute(request::Payload {
                commands: [request::PayloadCommand {
                    devices: [
                        request::PayloadCommandDevice {
                            id: String::from("123"),
                            custom_data: json!({
                                "fooValue": 74,
                                "barValue": true,
                                "bazValue": "sheepdip"
                            })
                            .as_object()
                            .unwrap()
                            .clone(),
                        },
                        request::PayloadCommandDevice {
                            id: String::from("456"),
                            custom_data: json!({
                                "fooValue": 36,
                                "barValue": false,
                                "bazValue": "moarsheep"
                            })
                            .as_object()
                            .unwrap()
                            .clone(),
                        },
                    ]
                    .to_vec(),
                    execution: [
                        request::PayloadCommandExecution {
                            command: Command::OnOff(commands::OnOff { on: true }),
                        },
                        request::PayloadCommandExecution {
                            command: Command::ColorAbsolute(commands::ColorAbsolute {
                                color: commands::Color {
                                    name: Some("magenta".to_string()),
                                    value: commands::ColorValue::Hsv {
                                        spectrum_hsv: commands::Hsv {
                                            hue: 300.0,
                                            saturation: 1.0,
                                            value: 1.0,
                                        },
                                    },
                                },
                            }),
                        },
                    ]
                    .to_vec(),
                }]
                .to_vec(),
            })]
            .to_vec(),
        },
    );
}

#[test]
fn execute_response() {
    common::two_way_serde(
        include_str!("json/execute/response.json"),
        Response {
            request_id: String::from("ff36a3cc-ec34-11e6-b1a0-64510650abcf"),
            payload: response::Payload {
                error_code: None,
                debug_string: None,
                commands: [
                    response::PayloadCommand {
                        ids: [String::from("123")].to_vec(),
                        status: response::PayloadCommandStatus::Success,
                        states: json!({
                            "on": true,
                            "online": true
                        })
                        .as_object()
                        .unwrap()
                        .to_owned(),
                        error_code: None,
                    },
                    response::PayloadCommand {
                        ids: [String::from("456")].to_vec(),
                        status: response::PayloadCommandStatus::Error,
                        states: Default::default(),
                        error_code: Some(String::from("deviceTurnedOff")),
                    },
                ]
                .to_vec(),
            },
        },
    );
}
