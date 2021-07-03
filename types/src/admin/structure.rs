use super::AddResponseError;

pub mod add {
    use super::AddResponseError;
    use crate::StructureID;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Clone, Deserialize, Serialize, Validate)]
    pub struct Request {
        pub structure_name: String,
    }

    pub type Response = Result<ResponseBody, AddResponseError>;

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct ResponseBody {
        pub structure_id: StructureID,
    }
}
