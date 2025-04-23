use ::axum::{Json, extract::State};
use ::std::sync::Arc;

use crate::{app::*, models::Permission, repositories::permissions::PermissionsRepository};

#[utoipa::path(
    get,
    path = "/v1/admin/permissions",
    tag = super::TAG_PERMISSIONS,
    params(
        ("Authorization" = String, Header, description = "JWT Bearer", example = json!(["Bearer <access token>"])),
    ),
    responses(
        (
            status = OK,
            description = "Permissions list",
            body = Vec<Permission>
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
    state.database.permissions_list().await.map(Json)
}
