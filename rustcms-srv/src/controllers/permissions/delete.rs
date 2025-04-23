use ::axum::extract::{State, Path};
use ::std::sync::Arc;

use crate::{app::*, repositories::permissions::PermissionsRepository};

#[utoipa::path(
    delete,
    path = "/v1/admin/permissions/{id}",
    tag = super::TAG_PERMISSIONS,
    params(
        ("Authorization" = String, Header, description = "JWT Bearer", example = json!(["Bearer <access token>"])),
        ("id" = String, description = "Permission ID"),
    ),
    responses(
        (
            status = OK,
            description = "Permission was deleted",
            body = ()
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
    security(("permission" = ["system:delete"])),
)]
#[handler(claims, permission = "system:delete", result)]
pub async fn delete(State(state): State<Arc<AppState>>, Path(id): Path<String>) {
    state.database.delete_permission(id).await?;
}
