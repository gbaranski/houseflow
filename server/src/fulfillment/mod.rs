pub mod ghome;
pub mod internal;

use std::time::Duration;

const EXECUTE_TIMEOUT: Duration = Duration::from_secs(5);
const QUERY_TIMEOUT: Duration = Duration::from_secs(5);
