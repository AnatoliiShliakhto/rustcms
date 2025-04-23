use ::axum::{Json, extract::State};
use ::std::sync::Arc;

use crate::{app::*, models::Role, repositories::roles::RolesRepository};

#[utoipa::path(
    get,
    path = "/v1/admin/roles",
    tag = super::TAG_ROLES,
    params(
        ("Authorization" = String, Header, description = "JWT Bearer", example = json!(["Bearer <access token>"])),
    ),
    responses(
        (
            status = OK,
            description = "Roles list",
            body = Vec<Role>
        ),
        (
            status = UNAUTHORIZED,
            description = "Invalid access token",
            body = ErrorBody,
        ),
        (
            status = FORBIDDEN,
            description = "Access forbidden",
            body = ErrorBody,
        ),
    ),
    security(("permission" = ["system:view"])),
)]
#[handler(claims, permission = "system:view")]
pub async fn list(State(state): State<Arc<AppState>>) {
    state.database.roles_list().await.map(Json)
}
