use ::axum::{extract::State, http::header::SET_COOKIE, response::AppendHeaders};
use ::axum_extra::extract::CookieJar;
use ::std::sync::Arc;

use crate::{
    app::*,
    repositories::middleware::TokenRepository,
};

#[utoipa::path(
    delete,
    path = "/v1/auth",
    tag = super::TAG_AUTHORIZATION,
    responses(
        (
            status = OK, 
            description = "Refresh token was revoked",
        ),
        (
            status = UNAUTHORIZED,
            description = "Invalid refresh token",
            body = ErrorBody,
        ),
        (
            status = BAD_REQUEST,
            description = "Payload isn't valid",
            body = ErrorBody,
        ),
    ),
)]
#[handler]
pub async fn revoke(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) {
    let Some(refresh_token) = jar.get("RT_UUID") else {
        Err(AuthError::MissingToken)?
    };

    if !refresh_token.value().is_empty() {
        state
            .database
            .delete_refresh_token(refresh_token.value())
            .await?;
    }

    Ok(
        AppendHeaders(vec![(
            SET_COOKIE,
            format!(
                "RT_UUID=; Path=/api/v1/auth; Expires=Thu, 01 Jan 1970 00:00:00 GMT; {}",
                state.config.security.set_cookie
            )
        )]))
}