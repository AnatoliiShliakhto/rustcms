use ::axum::http::StatusCode;

use super::TAG_UTILITIES;

#[utoipa::path(
    get,
    path = "/healthcheck",
    responses((status = OK, description = "Endpoint for k8s healthcheck functionality")),
    tag = TAG_UTILITIES,
)]
pub async fn healthcheck() -> StatusCode {
    StatusCode::OK
}
