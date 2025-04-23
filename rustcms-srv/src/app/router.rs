use ::axum::{Router, extract::DefaultBodyLimit, http::Method, middleware::from_fn_with_state};
use ::std::sync::Arc;
use ::tower_http::{compression::CompressionLayer, cors::CorsLayer, services::ServeDir};

use crate::{
    app::*,
    services::middleware::{storage::private_storage, response::response_headers},
};
use super::api::init_api;

pub async fn init_app(state: &Arc<AppState>) -> Result<Router, Error> {
    let cors_layer = CorsLayer::new()
        .allow_origin([state.config.server.url.parse().unwrap()])
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
        .nest_service("/private", ServeDir::new(state.config.server.path.join("private")))
        .layer(from_fn_with_state(state.clone(), private_storage))
        .layer(from_fn_with_state(
            state.config.headers.private_storage.clone(),
            response_headers,
        ))
        .merge(init_api(state).await?)
        .layer(from_fn_with_state(
            state.config.headers.api.clone(),
            response_headers,
        ))
        .nest_service("/public", ServeDir::new(state.config.server.path.join("public")))
        .layer(from_fn_with_state(
            state.config.headers.public_storage.clone(),
            response_headers,
        ))
        .fallback_service(ServeDir::new(state.config.server.path.join("www")))
        .layer(from_fn_with_state(
            state.config.headers.common.clone(),
            response_headers,
        ))
        .layer(compression_layer)
        .layer(cors_layer)
        .layer(DefaultBodyLimit::max(state.config.security.max_body_limit));

    Ok(router)
}
