use crate::{extractors::UserID, State};
use axum::{extract, response};
use houseflow_types::{
    auth::whoami::{Request, Response},
    errors::{AuthError, ServerError},
};

#[tracing::instrument(name = "Whoami", skip(state), err)]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    UserID(user_id): UserID,
    extract::Json(_request): extract::Json<Request>,
) -> Result<response::Json<Response>, ServerError> {
    let user = state
        .database
        .get_user(&user_id)?
        .ok_or(AuthError::UserNotFound)?;

    tracing::info!(username = %user.username, email = %user.email);

    Ok(response::Json(Response {
        username: user.username,
        email: user.email,
    }))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use axum::{extract, response};

    #[tokio::test]
    async fn valid() {
        let state = get_state();
        let user = get_user();
        state.database.add_user(&user).unwrap();
        let response::Json(response) = super::handle(
            state.clone(),
            crate::extractors::UserID(user.id),
            extract::Json(super::Request {}),
        )
        .await
        .unwrap();
        assert_eq!(response.email, user.email);
        assert_eq!(response.username, user.username);
    }
}
