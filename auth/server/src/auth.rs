use crate::AppData;
use actix_files::NamedFile;
use actix_web::{
    get, post,
    web::{self, Data, Json, Query},
};
use houseflow_auth_types::{
    LoginError, LoginRequest, LoginResponseBody, RegisterError, RegisterRequest,
    RegisterResponseBody,
};
use houseflow_db::Database;
use houseflow_types::User;
use rand::random;

struct SomeWrapper([u8; 10]);

impl std::ops::Deref for SomeWrapper {
    type Target = [u8; 10];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[post("/login")]
pub async fn login(
    request: Json<LoginRequest>,
    app_data: Data<AppData>,
    db: Data<dyn Database>,
) -> Result<Json<LoginResponseBody>, LoginError> {
    let user = db.get_user_by_email(&request.email).await?;
    todo!()
}

#[post("/register")]
pub async fn register(
    request: Json<RegisterRequest>,
    app_data: Data<AppData>,
    db: Data<dyn Database>,
) -> Result<Json<RegisterResponseBody>, RegisterError> {
    todo!()
}
