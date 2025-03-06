use crate::app::*;

#[utoipa::path(
    get,
    path = "/healthcheck",
    responses((status = OK, description = "Endpoint for k8s healthcheck functionality")),
    tag = super::TAG_UTILITIES,
)]
#[handler(result)]
pub async fn healthcheck() {}
