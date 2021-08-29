use crate::get_with_token;
use crate::post_with_token;
use crate::Error;
use crate::HouseflowAPI;
use houseflow_types::errors::ServerError;
use houseflow_types::fulfillment;
use houseflow_types::token::AccessToken;

#[derive(Debug, thiserror::Error)]
pub enum FulfillmentError {}

impl HouseflowAPI {
    pub async fn sync(
        &self,
        access_token: &AccessToken,
    ) -> Result<Result<fulfillment::sync::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("sync").unwrap();
        get_with_token(url, &fulfillment::sync::Request {}, access_token).await
    }

    pub async fn execute(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::execute::Request,
    ) -> Result<Result<fulfillment::execute::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("execute").unwrap();
        post_with_token(url, request, access_token).await
    }

    pub async fn query(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::query::Request,
    ) -> Result<Result<fulfillment::query::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("query").unwrap();
        post_with_token(url, request, access_token).await
    }
}
