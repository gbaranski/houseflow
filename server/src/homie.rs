use homie_controller::Event;
use homie_controller::HomieController;
use homie_controller::HomieEventLoop;
use homie_controller::PollError;
use houseflow_types::user::Homie;
use rumqttc::ClientConfig;
use rumqttc::ConnectionError;
use rumqttc::MqttOptions;
use rumqttc::TlsConfiguration;
use rumqttc::Transport;
use std::sync::Arc;
use std::time::Duration;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time::sleep;

pub fn get_mqtt_options(
    config: &Homie,
    tls_client_config: Option<Arc<ClientConfig>>,
) -> MqttOptions {
    let mut mqtt_options = MqttOptions::new(&config.client_id, &config.host, config.port);
    mqtt_options.set_keep_alive(5);
    mqtt_options.set_clean_session(false);

    if let (Some(username), Some(password)) = (&config.username, &config.password) {
        mqtt_options.set_credentials(username, password);
    }

    if let Some(client_config) = tls_client_config {
        mqtt_options.set_transport(Transport::tls_with_config(TlsConfiguration::Rustls(
            client_config,
        )));
    }

    mqtt_options
}

pub fn spawn_homie_poller(
    controller: Arc<HomieController>,
    mut event_loop: HomieEventLoop,
    reconnect_interval: Duration,
) -> JoinHandle<()> {
    task::spawn(async move {
        loop {
            match controller.poll(&mut event_loop).await {
                Ok(Some(event)) => {
                    handle_homie_event(controller.as_ref(), event).await;
                }
                Ok(None) => {}
                Err(e) => {
                    tracing::error!(
                        "Failed to poll HomieController for base topic '{}': {}",
                        controller.base_topic(),
                        e
                    );
                    if let PollError::Connection(ConnectionError::Io(_)) = e {
                        sleep(reconnect_interval).await;
                    }
                }
            }
        }
    })
}

async fn handle_homie_event(_controller: &HomieController, event: Event) {
    match event {
        _ => tracing::info!("Homie event {:?}", event),
    }
}
