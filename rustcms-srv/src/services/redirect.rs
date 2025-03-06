use ::axum::{
    handler::HandlerWithoutStateExt,
    http::uri::{Authority, Scheme},
    http::{StatusCode, Uri},
    response::Redirect,
};
use ::axum_extra::extract::Host;
use ::axum_server::Handle;
use ::std::net::{IpAddr, Ipv4Addr, SocketAddr};
use ::tracing::{info, warn};

use crate::app::*;

pub async fn redirect_http_to_https(host: Ipv4Addr, ports: (u16, u16), handle: Handle) {
    fn make_https(host: &str, uri: Uri, https_port: u16) -> Result<Uri> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(Scheme::HTTPS);

        parts
            .path_and_query
            .is_none()
            .then(|| parts.path_and_query = Some("/".parse().unwrap()));

        let Ok(authority) = host.parse::<Authority>() else {
            Err(Error::CustomError("Invalid host"))?
        };
        let bare_host = match authority.port() {
            Some(port_struct) => authority
                .as_str()
                .strip_suffix(port_struct.as_str())
                .unwrap()
                .strip_suffix(':')
                .unwrap(),
            None => authority.as_str(),
        };

        let Ok(authority) = format!("{bare_host}:{https_port}").parse::<Authority>() else {
            Err(Error::CustomError("Invalid host"))?
        };

        parts.authority = Some(authority);

        let Ok(uri) = Uri::from_parts(parts) else {
            Err(Error::CustomError("Invalid URI"))?
        };

        Ok(uri)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(&host, uri, ports.1) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::new(IpAddr::V4(host), ports.0);
    info!("HTTP redirect server listening on {addr:?}");
    axum_server::bind(addr)
        .handle(handle)
        .serve(redirect.into_make_service())
        .await
        .unwrap();
}
