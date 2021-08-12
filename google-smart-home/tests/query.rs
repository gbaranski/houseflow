mod common;

use google_smart_home::{
    query::{
        request,
        response::{self, Response},
    },
    Request, RequestInput,
};
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
                            state: json!({
                                "on": true,
                                "online": true,
                            })
                            .as_object()
                            .unwrap()
                            .clone(),
                        },
                    ),
                    (
                        String::from("456"),
                        response::PayloadDevice {
                            status: response::PayloadDeviceStatus::Success,
                            error_code: None,
                            state: json!({
                                "on": true,
                                "online": true,
                                "brightness": 80,
                                "color": {
                                    "name": "cerulean",
                                    "spectrumRGB": 31655
                                },
                            })
                            .as_object()
                            .unwrap()
                            .clone(),
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
