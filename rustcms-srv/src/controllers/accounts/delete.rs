use ::axum::extract::State;
use ::std::sync::Arc;

use crate::{app::*, repositories::accounts::AccountsRepository, services::middleware::*};

#[utoipa::path(
    delete,
    path = "/v1/account",
    tag = super::TAG_ACCOUNT,
    params(
        ("Authorization" = String, Header, description = "JWT Bearer", example = json!(["Bearer <access token>"])),
    ),
    responses(
        (
            status = OK,
            description = "Account was deleted",
        ),
        (
            status = UNAUTHORIZED,
            description = "Invalid access token",
            body = ErrorBody,
        ),
    ),
)]
#[handler(result)]
pub async fn delete(State(state): State<Arc<AppState>>, claims: Claims<'_>) {
    state.database.delete_account(claims.auth.unwrap().id).await?;
}
