use actix_web::{
    get,
    web::{Data, Json},
    HttpRequest,
};
use config::server::Secrets;
use db::Database;
use fulfillment_types::{
    ExecuteRequest, ExecuteResponse, ExecuteResponseBody, ExecuteResponseError,
};
use lighthouse_api::prelude::Lighthouse;
use token::Token;
use types::{DevicePermission, UserAgent};

const USER_AGENT: UserAgent = UserAgent::Internal;

const EXECUTE_PERMISSION: DevicePermission = DevicePermission {
    read: true,
    write: false,
    execute: true,
};

#[get("/execute")]
pub async fn on_execute(
    execute_request: Json<ExecuteRequest>,
    http_request: HttpRequest,
    secrets: Data<Secrets>,
    db: Data<dyn Database>,
    lighthouse: Data<dyn Lighthouse>,
) -> Result<Json<ExecuteResponse>, ExecuteResponseError> {
    let access_token = Token::from_request(&http_request)?;
    access_token.verify(&secrets.access_key, Some(&USER_AGENT))?;
    if !db
        .check_user_device_permission(
            access_token.user_id(),
            &execute_request.device_id,
            &EXECUTE_PERMISSION,
        )
        .await?
    {
        return Err(ExecuteResponseError::NoDevicePermission);
    }

    let response_frame = match lighthouse
        .execute(&execute_request.frame, &execute_request.device_id)
        .await
    {
        Ok(frame) => frame,
        Err(err) => match err {
            lighthouse_api::Error::DeviceError(err) => return Err(err.into()),
            lighthouse_api::Error::ReqwestError(err) => {
                return Err(ExecuteResponseError::InternalError(err.to_string()))
            }
        },
    };

    Ok(Json(ExecuteResponse::Ok(ExecuteResponseBody {
        frame: response_frame,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use actix_web::{http, test, App};
    use lighthouse_proto::{execute, execute_response};
    use std::sync::Arc;
    use types::{DeviceCommand, DeviceError, DeviceStatus};

    #[actix_rt::test]
    async fn execute() {
        let database = get_database();
        let secrets: Secrets = rand::random();
        let (lighthouse, mut request_receiver, response_sender) = get_lighthouse();
        let lighthouse = lighthouse;
        let actix_lighthouse = Data::from(lighthouse.clone() as Arc<dyn Lighthouse>);
        let user = get_user();
        let device = get_device();
        let access_token =
            Token::new_access_token(&secrets.access_key, &user.id, &UserAgent::Internal);
        database.add_user(&user).await.unwrap();
        database.add_device(&device).await.unwrap();
        database
            .add_user_device(&device.id, &user.id, &EXECUTE_PERMISSION)
            .await
            .unwrap();

        let mut app = test::init_service(App::new().configure(|cfg| {
            crate::configure(cfg, database, actix_lighthouse.clone(), secrets.clone())
        }))
        .await;

        let request_frame = execute::Frame {
            id: rand::random(),
            command: DeviceCommand::OnOff,
            params: Default::default(),
        };

        let response_frame = execute_response::Frame {
            id: request_frame.id.clone(),
            status: DeviceStatus::Success,
            error: DeviceError::None,
            state: Default::default(),
        };

        let request_body = ExecuteRequest {
            device_id: device.id.clone(),
            frame: request_frame.clone(),
        };

        let request = test::TestRequest::get()
            .uri("/internal/execute")
            .insert_header((
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token.to_string()),
            ))
            .set_json(&request_body)
            .to_request();

        tokio::join!(
            async {
                let request_frame_recv = request_receiver.recv().await.unwrap();
                assert_eq!(request_frame, request_frame_recv);
                response_sender.send(response_frame.clone()).await.unwrap();
            },
            async {
                let response = test::call_service(&mut app, request).await;
                assert_eq!(
                    response.status(),
                    200,
                    "status is not succesfull, body: {:?}",
                    test::read_body(response).await
                );
                // TODO: implement testing with mcoked lighthouse
                let response: ExecuteResponseBody = test::read_body_json(response).await;
                assert_eq!(response.frame, response_frame);
            },
        );
    }
}
