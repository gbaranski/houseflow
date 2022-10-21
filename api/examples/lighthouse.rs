use houseflow_api::server::lighthouse;

#[tokio::main]
async fn main() {
    lighthouse::connect("127.0.0.1:8080").await.unwrap();
}