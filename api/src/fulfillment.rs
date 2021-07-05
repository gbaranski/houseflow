use crate::{get_with_token, post_with_token, Error, HouseflowAPI};
use houseflow_types::{fulfillment, token::AccessToken};

#[derive(Debug, thiserror::Error)]
pub enum FulfillmentError {}

impl HouseflowAPI {
    pub async fn sync(
        &self,
        access_token: &AccessToken,
    ) -> Result<fulfillment::sync::Response, Error> {
        let url = self.fulfillment_url.join("sync").unwrap();
        get_with_token(url, &fulfillment::sync::Request {}, access_token).await
    }

    pub async fn execute(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::execute::Request,
    ) -> Result<fulfillment::execute::Response, Error> {
        let url = self.fulfillment_url.join("execute").unwrap();
        post_with_token(url, request, access_token).await
    }

    pub async fn query(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::query::Request,
    ) -> Result<fulfillment::query::Response, Error> {
        let url = self.fulfillment_url.join("query").unwrap();
        get_with_token(url, request, access_token).await
    }
}
