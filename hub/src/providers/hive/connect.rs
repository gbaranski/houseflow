use super::provider;
use super::session;
use crate::providers::Event;
use crate::providers::EventSender;
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::Extension;
use axum::extract::TypedHeader;
use axum::headers;
use axum::http::StatusCode;
use axum::response::Response;
use futures::StreamExt;
use houseflow_types::accessory;
use serde::Deserialize;
use serde::Serialize;

pub struct DeviceCredentials(accessory::ID, accessory::Password);

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "error", content = "description")]
pub enum ConnectError {
    InvalidAuthorizationHeader(String),
    AccessoryNotFound,
    AccessoryAlreadyConnected,
}

impl axum::response::IntoResponse for ConnectError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::InvalidAuthorizationHeader(_) => StatusCode::BAD_REQUEST,
            Self::AccessoryNotFound => StatusCode::UNAUTHORIZED,
            Self::AccessoryAlreadyConnected => StatusCode::NOT_ACCEPTABLE,
        };
        let mut response = axum::Json(self).into_response();
        *response.status_mut() = status;

        response
    }
}

#[async_trait]
impl axum::extract::FromRequest<Body> for DeviceCredentials {
    type Rejection = axum::Json<ConnectError>;

    async fn from_request(
        req: &mut axum::extract::RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        tracing::info!("hello world 1");

        let TypedHeader(headers::Authorization(authorization)) =
            TypedHeader::<headers::Authorization<headers::authorization::Basic>>::from_request(req)
                .await
                .map_err(|err| ConnectError::InvalidAuthorizationHeader(err.to_string()))?;
        let accessory_id = accessory::ID::parse_str(authorization.username()).map_err(|err| {
            ConnectError::InvalidAuthorizationHeader(format!("invalid hub id: {}", err))
        })?;

        Ok(Self(accessory_id, authorization.password().to_owned()))
    }
}

pub async fn websocket_handler(
    websocket: axum::extract::ws::WebSocketUpgrade,
    Extension(mut hive_provider): Extension<provider::Address>,
    Extension(global_events): Extension<EventSender>,
    DeviceCredentials(accessory_id, password): DeviceCredentials,
) -> Result<impl axum::response::IntoResponse, ConnectError> {
    let accessory = hive_provider
        .send(provider::messages::GetAccessoryConfiguration { accessory_id })
        .await
        .unwrap()
        .ok_or(ConnectError::AccessoryNotFound)?;
    if hive_provider
        .send(provider::messages::IsConnected { accessory_id })
        .await
        .unwrap()
    {
        return Err(ConnectError::AccessoryAlreadyConnected);
    }
    // TODO: Verify password
    Ok(websocket.on_upgrade(move |stream| async move {
        let (tx, mut rx) = stream.split();
        let mut session: session::Address = hive_provider
            .send(provider::messages::Connected {
                sink: tx,
                accessory_id,
            })
            .await
            .unwrap()
            .unwrap();
        global_events.send(Event::Connected { accessory }).unwrap();
        {
            let session_cloned = session.clone();
            let events_cloned = global_events.clone();
            tokio::select! {
                _ = tokio::spawn(async move {
                    while let Some(message) = rx.next().await {
                        let message = message?;
                        let response = session.send(message).await??;
                        if let Some(event) = response {
                            events_cloned.send(event)?;
                        }

                    }
                    Ok::<(), anyhow::Error>(())
                }) => {
                    tracing::debug!("stream has been closed");
                }
                _ = tokio::spawn(async move { session_cloned.wait_for_stop().await; }) => {
                    tracing::debug!("actor has stopped");
                }

            }
        }
        tracing::info!("session closed");
        hive_provider
            .send(provider::messages::Disconnected { accessory_id })
            .await?;
        global_events.send(Event::Disconnected { accessory_id })?;
        Ok::<_, anyhow::Error>(())
    }))
}
