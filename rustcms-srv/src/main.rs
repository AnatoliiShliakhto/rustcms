#![forbid(unsafe_code)]
#![deny(clippy::all)]
mod app;
mod controllers;
mod models;
mod repositories;
mod services;

use ::std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use ::tracing::info;

use crate::{
    app::*,
    services::{redirect::redirect_http_to_https, shutdown::shutdown_handler},
};

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    print!(include_str!("../resources/logo.txt"));

    let state = Arc::new(AppState::init().await?);

    let (ssl_crt_path, ssl_key_path) = (
        PathBuf::from(&state.config.server.path)
            .join("cert")
            .join("ssl.crt "),
        PathBuf::from(&state.config.server.path)
            .join("cert")
            .join("private.key"),
    );

    let https_enabled = ssl_crt_path.exists() & ssl_key_path.exists();

    https_enabled.then(|| {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .expect("Crypto Provider initialization failed.")
    });

    let handle = axum_server::Handle::new();
    tokio::spawn(shutdown_handler(handle.clone()));

    let app = router::init_app(&state).await?.into_make_service();

    if https_enabled {
        tokio::spawn(redirect_http_to_https(
            state.config.server.host,
            (
                state.config.server.http_port,
                state.config.server.https_port,
            ),
            handle.clone(),
        ));

        let addr = SocketAddr::new(
            IpAddr::V4(state.config.server.host),
            state.config.server.https_port,
        );
        let tls_cfg =
            axum_server::tls_rustls::RustlsConfig::from_pem_file(ssl_crt_path, ssl_key_path)
                .await
                .map_err(|_| Box::new(Error::CustomError("Error loading TLS certificate")))?;

        info!("HTTPS server listening on {addr:?}");
        axum_server::bind_rustls(addr, tls_cfg)
            .handle(handle)
            .serve(app)
            .await
            .map_err(|_| Box::new(Error::CustomError("Error starting HTTPS server")))?;
    } else {
        let addr = SocketAddr::new(
            IpAddr::V4(state.config.server.host),
            state.config.server.http_port,
        );

        info!("HTTP server listening on {addr:?}");
        axum_server::bind(addr)
            .handle(handle)
            .serve(app)
            .await
            .map_err(|_| Box::new(Error::CustomError("Error starting HTTP server")))?;
    }

    info!("Server stopped");

    Ok(())
}
