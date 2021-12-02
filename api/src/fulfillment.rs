use crate::Error;
use crate::Client;
use houseflow_types::errors::ServerError;
use houseflow_types::fulfillment;
use houseflow_types::token::AccessToken;

impl Client {
    pub async fn sync(
        &self,
        access_token: &AccessToken,
    ) -> Result<Result<fulfillment::sync::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("sync").unwrap();
        self.get_with_token(url, &fulfillment::sync::Request {}, access_token).await
    }

    pub async fn execute(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::execute::Request,
    ) -> Result<Result<fulfillment::execute::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("execute").unwrap();
        self.post_with_token(url, request, access_token).await
    }

    pub async fn query(
        &self,
        access_token: &AccessToken,
        request: &fulfillment::query::Request,
    ) -> Result<Result<fulfillment::query::Response, ServerError>, Error> {
        let url = self.fulfillment_url.join("query").unwrap();
        self.post_with_token(url, request, access_token).await
    }
}
