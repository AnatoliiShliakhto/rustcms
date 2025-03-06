use ::axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};

pub async fn response_headers(
    State(headers): State<HeaderMap>,
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;

    for (header_name, header_value) in headers.iter() {
        if response.headers().contains_key(header_name) {
            continue;
        }

        response
            .headers_mut()
            .insert(header_name, header_value.clone());
    }

    response
}
