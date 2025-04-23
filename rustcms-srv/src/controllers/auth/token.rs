use ::axum::{extract::State, http::header::SET_COOKIE, response::AppendHeaders, Json};
use ::axum_extra::extract::CookieJar;
use ::serde::Serialize;
use ::std::{borrow::Cow, sync::Arc};
use ::utoipa::ToSchema;

use crate::{
    app::*,
    repositories::{accounts::AccountAuthRepository, middleware::TokenRepository},
    services::middleware::*,
};

#[derive(Serialize, ToSchema)]
pub struct RefreshTokenBody<'a> {
    pub token_type: &'a str,
    pub access_token: Cow<'a, str>,
}

#[utoipa::path(
    get,
    path = "/v1/auth/token",
    tag = super::TAG_AUTHORIZATION,
    responses(
        (
            status = OK, 
            description = "Returns the authorized tokens",
            body = RefreshTokenBody,
        ),
        (
            status = UNAUTHORIZED,
            description = "The Refresh Token is invalid or expired",
            body = ErrorBody,
        ),
        (
            status = BAD_REQUEST,
            description = "The Payload isn't valid",
            body = ErrorBody,
        ),
    ),
)]
#[handler]
pub async fn token(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) {
    let Some(current_refresh_token) = jar.get("RT_UUID") else {
        Err(AuthError::MissingToken)?
    };

    if current_refresh_token.value().is_empty() {
        Err(AuthError::InvalidToken)?
    }

    let auth = state
        .database
        .find_auth_by_token(current_refresh_token.value())
        .await?;

    let refresh_token = state
        .database
        .update_refresh_token(current_refresh_token.value(), state.config.security.jwt.refresh_expiration)
        .await?;

    // Build the access token
    let access_token = Claims::new()
        .issuer(&state.config.security.jwt.issuer)
        .subject(&state.config.security.jwt.subject)
        .expiration_minutes(state.config.security.jwt.access_expiration)
        .auth(auth)
        .build_token(&state.config.security.jwt.keys.encoding)?;

    Ok((
        AppendHeaders(vec![(
            SET_COOKIE,
            format!(
                "RT_UUID={refresh_token}; Path=/api/v1/auth; Max-Age={0}; {1}",
                state.config.security.jwt.refresh_expiration * 24 * 60 * 60,
                state.config.security.set_cookie,
            )
        )]),
        Json(RefreshTokenBody {
            token_type: "Bearer",
            access_token,
        })))
}
