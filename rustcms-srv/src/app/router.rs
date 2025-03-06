use ::axum::{Router, extract::DefaultBodyLimit, http::Method, middleware::from_fn_with_state};
use ::std::sync::Arc;
use ::tower_http::{compression::CompressionLayer, cors::CorsLayer, services::ServeDir};

use crate::{
    app::*,
    services::middleware::{private_storage, response_headers},
};

use super::api::init_api;

pub async fn init_app(state: &Arc<AppState>) -> Result<Router> {
    let cors_layer = CorsLayer::new()
        .allow_origin([state.cfg.url.parse().unwrap()])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_credentials(true);

    let compression_layer = CompressionLayer::new().br(true).gzip(true).zstd(true);

    let router = Router::new()
        .nest_service("/private", ServeDir::new(state.cfg.path.join("private")))
        .layer(from_fn_with_state(state.clone(), private_storage))
        .layer(from_fn_with_state(
            state.cfg.private_storage_response_headers.clone(),
            response_headers,
        ))
        .merge(init_api(state).await?)
        .layer(from_fn_with_state(
            state.cfg.api_response_headers.clone(),
            response_headers,
        ))
        .nest_service("/public", ServeDir::new(state.cfg.path.join("public")))
        .layer(from_fn_with_state(
            state.cfg.public_storage_response_headers.clone(),
            response_headers,
        ))
        .fallback_service(ServeDir::new(state.cfg.path.join("www")))
        .layer(from_fn_with_state(
            state.cfg.static_response_headers.clone(),
            response_headers,
        ))
        .layer(compression_layer)
        .layer(cors_layer)
        .layer(DefaultBodyLimit::max(state.cfg.max_body_limit));

    Ok(router)
}
