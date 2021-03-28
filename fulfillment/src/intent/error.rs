#[derive(Debug)]
pub enum IntentError {
    /// Occurs when there is no correspoding entry for specific device and user
    NoDevicePermission,

    /// Occurs when there is corresponding entry for permission, but .execute is false
    NoDeviceExecutePermission,

    /// Occurs when there is no entry for corresponding device ID in memcached
    NoWealthyLighthouse,

    DatabaseError(houseflow_db::Error),
    LighthouseError(houseflow_lighthouse::Error),
}

impl From<houseflow_db::Error> for IntentError {
    fn from(err: houseflow_db::Error) -> Self {
        Self::DatabaseError(err)
    }
}

impl From<houseflow_lighthouse::Error> for IntentError {
    fn from(err: houseflow_lighthouse::Error) -> Self {
        Self::LighthouseError(err)
    }
}

impl std::fmt::Display for IntentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use IntentError::*;

        let msg = match self {
            NoDevicePermission => String::from("relinkRequired"),
            NoDeviceExecutePermission => String::from("authFailure"),
            NoWealthyLighthouse => String::from("deviceOffline"),
            DatabaseError(_) => String::from("hardError"),
            LighthouseError(_) => String::from("hardError"),
        };

        write!(f, "{}", msg)
    }
}
