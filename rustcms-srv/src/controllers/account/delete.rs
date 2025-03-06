use ::axum::extract::State;
use ::std::sync::Arc;

use crate::{app::*, repositories::account::AccountRepository, services::middleware::Claims};

#[utoipa::path(
    delete,
    path = "/v1/account/delete",
    tag = super::TAG_ACCOUNT,
    params(
        ("Authorization" = String, Header, description = "Access token", example = json!(["Bearer <token>"])),
    ),
    responses(
        (
            status = OK,
            description = "Account was deleted",
        ),
        (
            status = UNAUTHORIZED,
            description = "Access token is expired",
            body = ErrorBody,
        ),
    ),
)]
#[handler(result)]
pub async fn delete(State(state): State<Arc<AppState>>, claims: Claims<'_>) {
    state.db.delete_account(claims.auth.unwrap().id).await?;
}
