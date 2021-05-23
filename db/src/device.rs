use semver::Version;
use houseflow_types::{Device, DeviceID};
use crate::{Database, Error};

impl Database {
    pub async fn get_device_by_id(&self, device_id: DeviceID) -> Result<Option<Device>, Error> {
        const QUERY: &str = "SELECT * FROM devices WHERE id = $1";
        let connection = self.pool.get().await?;
        let row = match connection.query_opt(QUERY, &[&device_id]).await? {
            Some(row) => row,
            None => return Ok(None),
        };

        let device = Device {
            id: row.try_get("id")?,
            password_hash: row.try_get("password_hash")?,
            device_type: row.try_get("type")?,
            traits: row.try_get("traits")?,
            name: row.try_get("name")?,
            will_push_state: row.try_get("will_push_state")?,
            room: row.try_get("room")?,
            model: row.try_get("model")?,
            hw_version: Version::parse(row.try_get("hw_version")?).map_err(|err| {
                Error::InvalidColumn {
                    column: "hw_version",
                    error: err.into(),
                }
            })?,
            sw_version: Version::parse(row.try_get("sw_version")?).map_err(|err| {
                Error::InvalidColumn {
                    column: "sw_version",
                    error: err.into(),
                }
            })?,
            attributes: row.try_get("attributes")?,
        };

        Ok(Some(device))
    }
}
