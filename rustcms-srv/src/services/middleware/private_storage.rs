use ::axum::{extract::Request, middleware::Next};

use crate::app::*;

#[handler(state, claims, permission = "storage:view")]
pub async fn private_storage(request: Request, next: Next) {
    Ok(next.run(request).await)
}
