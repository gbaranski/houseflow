use super::AddResponseError;

pub mod add {
    use super::AddResponseError;
    use crate::{StructureID, UserID};
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Clone, Deserialize, Serialize, Validate)]
    pub struct Request {
        pub structure_id: StructureID,
        pub user_id: UserID,
        pub is_manager: bool,
    }

    pub type Response = Result<ResponseBody, ResponseError>;
    pub type ResponseError = AddResponseError;

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct ResponseBody {}
}
