mod common;

use google_smart_home::query::request;
use google_smart_home::query::response;
use google_smart_home::query::response::Response;
use google_smart_home::Request;
use google_smart_home::RequestInput;
use serde_json::json;

#[test]
fn query_request() {
    common::two_way_serde(
        include_str!("json/query/request.json"),
        Request {
            request_id: String::from("ff36a3cc-ec34-11e6-b1a0-64510650abcf"),
            inputs: [RequestInput::Query(request::Payload {
                devices: [
                    request::PayloadDevice {
                        id: String::from("123"),
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
                    },
                    request::PayloadDevice {
                        id: String::from("456"),
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
                    },
                ]
                .to_vec(),
            })]
            .to_vec(),
        },
    );
}

#[test]
fn query_response() {
    common::two_way_serde(
        include_str!("json/query/response.json"),
        Response {
            request_id: String::from("ff36a3cc-ec34-11e6-b1a0-64510650abcf"),
            payload: response::Payload {
                error_code: None,
                debug_string: None,
                devices: [
                    (
                        String::from("123"),
                        response::PayloadDevice {
                            status: response::PayloadDeviceStatus::Success,
                            error_code: None,
                            state: response::State {
                                online: true,
                                on: Some(true),
                                ..Default::default()
                            },
                        },
                    ),
                    (
                        String::from("456"),
                        response::PayloadDevice {
                            status: response::PayloadDeviceStatus::Success,
                            error_code: None,
                            state: response::State {
                                online: true,
                                on: Some(true),
                                brightness: Some(80),
                                color: Some(response::Color::SpectrumRgb(31655)),
                                ..Default::default()
                            },
                        },
                    ),
                ]
                .iter()
                .cloned()
                .collect(),
            },
        },
    );
}
