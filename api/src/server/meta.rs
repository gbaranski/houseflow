use super::Client;
use crate::Error;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::errors::ServerError;
use reqwest::Url;

impl Client {
    fn meta_url(&self, path: &str) -> Url {
        self.config
            .server
            .url
            .join(&format!("controllers/meta/{}", path))
            .unwrap()
    }

    pub async fn read_characteristics(
        &self,
        accessory_id: &accessory::ID,
        service_name: &ServiceName,
        characteristic_name: &CharacteristicName,
    ) -> Result<Result<Characteristic, ServerError>, Error> {
        let url = self.meta_url(&format!(
            "characteristic/{}/{}/{}",
            accessory_id, service_name, characteristic_name
        ));
        self.get(url, &()).await
    }

    pub async fn write_characteristics(
        &self,
        accessory_id: &accessory::ID,
        service_name: &ServiceName,
        characteristic: &Characteristic,
    ) -> Result<Result<(), ServerError>, Error> {
        let url = self.meta_url(&format!("characteristic/{}/{}", accessory_id, service_name));
        self.get(url, characteristic).await
    }
}
