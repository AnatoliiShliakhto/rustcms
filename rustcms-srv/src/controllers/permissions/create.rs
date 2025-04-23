use ::axum::{Json, extract::State};
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
pub struct CreatePermissionPayload<'a> {
    #[validate(length(min = 4, max = 30, message = "must be between 4 and 30 characters"))]
    pub name: Cow<'a, str>,
}

#[utoipa::path(
    post,
    path = "/v1/admin/permissions",
    tag = super::TAG_PERMISSIONS,
    params(
        ("Authorization" = String, Header, description = "JWT Bearer", example = json!(["Bearer <access token>"])),
    ),
    request_body(
        description = "Create permission",
        content = CreatePermissionPayload,
    ),
    responses(
        (
            status = OK,
            description = "Permission created",
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
    security(("permission" = ["system:create"])),
)]
#[handler(permission = "system:create")]
pub async fn create(
    State(state): State<Arc<AppState>>,
    claims: Claims<'_>,
    ValidatedJson(payload): ValidatedJson<CreatePermissionPayload<'_>>,
) {
    state
        .database
        .create_permission(payload.name, claims.account_id())
        .await
        .map(Json)
}
