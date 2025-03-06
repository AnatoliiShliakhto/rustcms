use axum::extract::{Request, State};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;

async fn insert_response_headers(
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
