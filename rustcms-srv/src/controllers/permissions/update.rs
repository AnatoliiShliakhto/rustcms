use ::axum::{
    Json,
    extract::{Path, State},
};
use ::serde::Deserialize;
use ::std::borrow::Cow;
use ::std::sync::Arc;
use ::utoipa::ToSchema;
use ::validator::Validate;

use crate::{
    app::*, models::Permission, repositories::permissions::PermissionsRepository,
    services::middleware::*,
};

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdatePermissionPayload<'a> {
    #[validate(length(min = 4, max = 30, message = "must be between 4 and 30 characters"))]
    pub name: Cow<'a, str>,
}

#[utoipa::path(
    put,
    path = "/v1/admin/permissions/{id}",
    tag = super::TAG_PERMISSIONS,
    params(
        ("Authorization" = String, Header, description = "JWT Bearer", example = json!(["Bearer <access token>"])),
        ("id" = String, description = "Permission ID"),
    ),
    request_body(
        description = "Update permission",
        content = UpdatePermissionPayload,
    ),
    responses(
        (
            status = OK,
            description = "Permission updated",
            body = Permission
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
    security(("permission" = ["system:edit"])),
)]
#[handler(permission = "system:edit")]
pub async fn update(
    State(state): State<Arc<AppState>>,
    claims: Claims<'_>,
    Path(id): Path<String>,
    ValidatedJson(payload): ValidatedJson<UpdatePermissionPayload<'_>>,
) {
    state
        .database
        .update_permission(id, payload.name, claims.account_id())
        .await
        .map(Json)
}
