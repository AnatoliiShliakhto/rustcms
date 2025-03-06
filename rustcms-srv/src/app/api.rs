use ::axum::Extension;
use ::axum::Router;
use ::std::sync::Arc;
use ::utoipa::OpenApi;
use ::utoipa_axum::{router::OpenApiRouter, routes};
//use ::utoipa_rapidoc::RapiDoc;
//use ::utoipa_redoc::{Redoc, Servable};
use ::utoipa_scalar::{Scalar, Servable as ScalarServable};

use crate::{
    app::*,
    controllers::*,
    services::middleware::Claims,
};

#[derive(utoipa::OpenApi)]
struct ApiDoc;

pub async fn init_api(state: &Arc<AppState>) -> Result<Router> {
    let api_endpoint = OpenApiRouter::new()
        .routes(routes!(account::register))
        .routes(routes!(account::delete))
        .routes(routes!(auth::authorize))
        .routes(routes!(auth::token))
        .routes(routes!(auth::revoke))
        .routes(routes!(utils::healthcheck))
        .layer(Extension(Arc::new(Claims::default())))
        .with_state(state.clone());

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", api_endpoint)
        .split_for_parts();

    let router = router
        //.merge(Redoc::with_url("/redoc", api.clone()))
        //.merge(RapiDoc::with_openapi("/api-docs/openapi.json", api.clone()).path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api));

    Ok(router)
}
