use crate::{extractors::UserID, State};
use axum::{extract::Extension, Json};
use houseflow_types::{
    auth::whoami::{Request, Response},
    errors::{AuthError, ServerError},
};

#[tracing::instrument(name = "Whoami", skip(state, _request), err)]
pub async fn handle(
    Extension(state): Extension<State>,
    UserID(user_id): UserID,
    Json(_request): Json<Request>,
) -> Result<Json<Response>, ServerError> {
    let user = state
        .database
        .get_user(&user_id)?
        .ok_or(AuthError::UserNotFound)?;

    tracing::info!(username = %user.username, email = %user.email);

    Ok(Json(Response {
        username: user.username,
        email: user.email,
    }))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use axum::Json;

    #[tokio::test]
    async fn valid() {
        let state = get_state();
        let user = get_user();
        state.database.add_user(&user).unwrap();
        let Json(response) = super::handle(
            state.clone(),
            crate::extractors::UserID(user.id),
            Json(super::Request {}),
        )
        .await
        .unwrap();
        assert_eq!(response.email, user.email);
        assert_eq!(response.username, user.username);
    }
}
