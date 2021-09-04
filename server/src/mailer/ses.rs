use super::Error;
use async_trait::async_trait;
use houseflow_config::server::EmailAwsSes as Config;
use lettre::Message;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_ses::RawMessage;
use rusoto_ses::SendRawEmailRequest;
use rusoto_ses::Ses;
use rusoto_ses::SesClient;

pub struct Mailer {
    ses_client: SesClient,
    config: Config,
}

impl Mailer {
    pub async fn new(config: Config) -> Self {
        let dispatcher = rusoto_core::HttpClient::new().unwrap();
        let credentials = rusoto_core::credential::ProfileProvider::with_default_configuration(
            &config.credentials,
        );
        dbg!(credentials.credentials().await).unwrap();
        let client = rusoto_core::Client::new_with(credentials, dispatcher);
        let ses_client = SesClient::new_with_client(client, config.region.clone());
        Self { ses_client, config }
    }
}

#[async_trait]
impl super::Mailer for Mailer {
    async fn send(&self, email: Message) -> Result<(), Error> {
        let request = SendRawEmailRequest {
            raw_message: RawMessage {
                data: base64::encode(email.formatted()).into(),
            },
            ..Default::default()
        };
        self.ses_client.send_raw_email(request).await?;
        Ok(())
    }

    fn from_address(&self) -> &str {
        &self.config.from
    }
}
