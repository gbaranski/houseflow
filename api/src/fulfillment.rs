use crate::{get_with_token, post_with_token, Error, HouseflowAPI};
use houseflow_types::{fulfillment, token::AccessToken, errors::ServerError};

#[derive(Debug, thiserror::Error)]
pub enum FulfillmentError {}

impl HouseflowAPI {
    pub async fn sync(
        &self,
        access_token: &AccessToken,
    ) -> Result<Result<fulfillment::internal::sync::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("sync").unwrap();
        get_with_token(url, &fulfillment::internal::sync::Request {}, access_token).await
    }

    pub async fn execute(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::internal::execute::Request,
    ) -> Result<Result<fulfillment::internal::execute::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("execute").unwrap();
        post_with_token(url, request, access_token).await
    }

    pub async fn query(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::internal::query::Request,
    ) -> Result<Result<fulfillment::internal::query::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("query").unwrap();
        get_with_token(url, request, access_token).await
    }
}
