mod login;
mod logout;
mod register;
mod token;
mod whoami;

pub use self::token::{on_exchange_refresh_token, on_exchange_refresh_token_form_config};
pub use login::on_login;
pub use logout::on_logout;
pub use register::on_register;
pub use whoami::on_whoami;

// #[cfg(test)]
// mod test_utils {
//     use super::TokenStore;
//     use token::store::MemoryTokenStore;

//     use actix_web::web::Data;
//     use std::sync::Arc;

//     pub const PASSWORD: &str = "SomePassword";
//     pub const PASSWORD_INVALID: &str = "SomeOtherPassword";
//     pub const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU";

//     pub fn get_token_store() -> Data<dyn TokenStore> {
//         Data::from(Arc::new(MemoryTokenStore::new()) as Arc<dyn TokenStore>)
//     }

//     pub fn get_database() -> Data<dyn db::Database> {
//         Data::from(Arc::new(db::memory::Database::new()) as Arc<dyn db::Database>)
//     }
// }
