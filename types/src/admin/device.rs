use super::AddResponseError;

pub mod add {
    use super::AddResponseError;
    use crate::{DeviceID, DeviceTrait, DeviceType, RoomID};
    use semver::Version;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Clone, Deserialize, Serialize, Validate)]
    pub struct Request {
        pub room_id: RoomID,

        #[validate(length(min = 8))]
        pub password: String,
        pub device_type: DeviceType,
        pub traits: Vec<DeviceTrait>,
        pub name: String,
        pub will_push_state: bool,
        pub model: String,
        pub hw_version: Version,
        pub sw_version: Version,
        pub attributes: serde_json::Map<String, serde_json::Value>,
    }

    pub type Response = Result<ResponseBody, AddResponseError>;

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct ResponseBody {
        pub device_id: DeviceID,
    }
}
