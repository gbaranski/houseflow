use super::AddResponseError;

pub mod add {
    use super::AddResponseError;
    use crate::{RoomID, StructureID};
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Clone, Deserialize, Serialize, Validate)]
    pub struct Request {
        pub room_name: String,
        pub structure_id: StructureID,
    }

    pub type Response = Result<ResponseBody, AddResponseError>;

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct ResponseBody {
        pub room_id: RoomID,
    }
}
