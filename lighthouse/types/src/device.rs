use actix::prelude::{Message, MessageResponse};
use actix_web::{dev::HttpResponseBuilder, http::StatusCode, HttpResponse};
use lighthouse_proto::{command, command_response, state, state_check, Frame};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

pub type BrokerResponse = Result<DeviceResponse, DeviceError>;

#[derive(Debug, Clone)]
pub enum DeviceRequest {
    Command(command::Frame),
    State(state_check::Frame),
}

impl Message for DeviceRequest {
    type Result = std::result::Result<DeviceResponse, DeviceError>;
}

impl Into<Frame> for DeviceRequest {
    fn into(self) -> Frame {
        match self {
            Self::Command(frame) => Frame::Command(frame),
            Self::State(frame) => Frame::StateCheck(frame),
        }
    }
}

#[derive(Debug, Clone, MessageResponse, Serialize, Deserialize)]
pub enum DeviceResponse {
    Command(command_response::Frame),
    State(state::Frame),
}

impl Into<Frame> for DeviceResponse {
    fn into(self) -> Frame {
        match self {
            Self::Command(frame) => Frame::CommandResponse(frame),
            Self::State(frame) => Frame::State(frame),
        }
    }
}

impl TryFrom<Frame> for DeviceResponse {
    type Error = ();

    fn try_from(frame: Frame) -> Result<Self, Self::Error> {
        match frame {
            Frame::CommandResponse(frame) => Ok(Self::Command(frame)),
            Frame::State(frame) => Ok(Self::State(frame)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub enum DeviceError {
    #[error("Device is not connected")]
    NotConnected,

    #[error("Timeout when sending request to device")]
    Timeout,
}

impl actix_web::ResponseError for DeviceError {
    fn error_response(&self) -> HttpResponse {
        let response = BrokerResponse::from(Err(self.clone()));
        let response_json = serde_json::to_string(&response).unwrap();
        HttpResponseBuilder::new(self.status_code()).body(response_json)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotConnected => StatusCode::NOT_FOUND,
            Self::Timeout => StatusCode::REQUEST_TIMEOUT,
        }
    }
}
